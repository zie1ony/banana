use odra::{Variable, types::{U256, Address, Balance}, Mapping, contract_env, UnwrapOrRevert, execution_error};
use odra_modules::{erc20::{Erc20, Erc20Ref}, access::Ownable};

/// A module definition. Each module struct consists Variables and Mappings
/// or/and another modules.
#[odra::module]
pub struct BananaToken {
    /// The module itself does not store the value, 
    /// it's a proxy that writes/reads value to/from the host.
    value: Variable<bool>,
    token: Erc20,
    ownable: Ownable
}

/// Module implementation.
/// 
/// To generate entrypoints,
/// an implementation block must be marked as #[odra::module].
#[odra::module]
impl BananaToken {
    /// Odra constructor.
    /// 
    /// Initializes the contract with the value of value.
    #[odra(init)]
    pub fn init(&mut self, approved_contract: Address) {
        let amount = U256::from(1000000);
        self.value.set(false);
        self.token.init("BananaToken".to_string(), "BNT".to_string(), 18, Some(amount));
        self.token.approve(approved_contract, U256::max_value());
        self.ownable.init();
    }

    delegate! {
        to self.token {
            pub fn transfer(&mut self, recipient: Address, amount: U256);
            pub fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256);
            pub fn approve(&mut self, spender: Address, amount: U256);
            pub fn name(&self) -> String;
            pub fn symbol(&self) -> String;
            pub fn decimals(&self) -> u8;
            pub fn total_supply(&self) -> U256;
            pub fn balance_of(&self, owner: Address) -> U256;
            pub fn allowance(&self, owner: Address, spender: Address) -> U256;
        }
    }
}

execution_error! {
    pub enum Error {
        WrongAmount => 30_000,
    }
}


#[odra::module]
pub struct MonkeyShow {
    map: Mapping<Address, U256>,
}

#[odra::module]
impl MonkeyShow {

    pub fn sell(&mut self, token_addr: Address, amount: U256, price: Balance) {
        Erc20Ref::at(token_addr).transfer_from(contract_env::caller(), contract_env::self_address(), amount);
        self.map.set(&token_addr, price);
    }

    #[odra(payable)]
    pub fn buy(&mut self, token_addr: Address, amount: U256) {
        let cspr = contract_env::attached_value();
        let price = self.map.get(&token_addr).unwrap_or_revert();
        if cspr > price * amount {
            contract_env::revert(Error::WrongAmount);
        }
    }
}

#[cfg(test)]
mod tests {
    use odra::test_env;

    use crate::banana_token::BananaTokenDeployer;

    #[test]
    fn flipping() {
        // To test a module we need to deploy it using autogenerated Deployer. 
        let alice = test_env::get_account(1);
        let mut contract = BananaTokenDeployer::init(alice);
    }
}
