//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! get it's current value [get_num] or [reset].
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
//use near_sdk::env::input;
use near_sdk::{log, near_bindgen, env, AccountId};

// standard imports to use dependenceis found in the Cargo.toml
// serializatoin imports bundle the code/storage for blockchain

use std::io::stdin; 
//use stdin to input account id from user


// allow web assembly and Near blockchain optimization
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
// rust smart contract patter = struct w/ associated impl where core logic resides
pub struct Counter {
    val: i8, //signed integer 8 bits - can use u8, u16, u32, u64, u128
    records: std::collections::HashMap<AccountId, String>,
    local_account_id: String,
}

#[near_bindgen]
impl Counter {

    /// Public method: Returns the counter value.
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    // function added by me to set signer account id 
    pub fn set_account_id(&mut self,message:String) {
        let mut input_string = String::new();
        log!("\nPlease input login id/name: ");
        stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
        // let account_id = AccountId::new_unchecked(format!("{}.{}", account_id, env::current_account_id()));

        let account_id = env::current_account_id(); //account id of contract caller
        log!("\nAccount ID contract set success! New contract ID = {}",account_id.to_string());

        self.local_account_id.insert_str(0, &input_string);
        log!("\nAccount local ID: on set call = {}",self.local_account_id);
        self.records.insert(account_id,message);

        if self.local_account_id.is_empty() == false
        {
            log!("\nAccount ID local set success! New local ID = {} This has NOT created a new near account with these credentials. This will come in a later contract example", self.local_account_id.to_string());
            return

        }

        else {
            log!("\nAccount ID local set failure. ID: {} = not set",self.local_account_id);
            return self.set_account_id(env::current_account_id().to_string());
        }

    }

    pub fn get_account_id(&self) -> Option<String> {
        let account_id = env::current_account_id();
        return Some(account_id.to_string());
    }

    /// Public method: Increment the counter.
    pub fn increment(&mut self) {
        self.val += 1;
        log!("\nIncreased number to {}", self.val);
        //println!("Increased number to {}", self.val);
    }

    /// Public method: Decrement the counter.
    pub fn decrement(&mut self) {
        self.val -= 1;
        log!("\nDecreased number to {}", self.val);
        //println!("Decreased number to {}", self.val);
    }

    /// Public method - Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        log!("\nReset counter to zero");
        //println!("Reset counter to zero");
    }

    pub fn display_account_id(&mut self) {
        log!("Contract Account ID: {:?}", self.get_account_id());
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn increment() {
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter::default();
        //let mut contract = Counter{val: 0, records: std::collections::HashMap::new()};
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract = Counter::default();
        //let mut contract = Counter{val: 0, records: std::collections::HashMap::new()};
        contract.decrement();
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let mut contract = Counter::default();
        //let mut contract = Counter{val: 0, records: std::collections::HashMap::new()};
        contract.increment();
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = Counter{val: 127, records: std::collections::HashMap::new(), local_account_id: "N/A".to_string()};
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panics_on_underflow() {
        let mut contract = Counter{val: -128, records: std::collections::HashMap::new(), local_account_id: "N/A".to_string()};
        contract.decrement();
    }
    #[test]
    fn get_account_info() {
        let mut contract = Counter::default();
        //let mut contract = Counter{val: 0, records: std::collections::HashMap::new()};
        contract.display_account_id();
        contract.get_account_id();
    
    }

    #[test]
    fn set_account_id() {
        log!("RUNNING SET ACCOUNT ID TEST - USER INPUT REQUIRED");
        let mut contract = Counter::default();
        //let mut contract = Counter{val: 0, records: std::collections::HashMap::new()};
        // let mut input_string = String::new();
        // log!("\nPlease input login id/name: ");
        // stdin().read_line(&mut input_string)
        // .ok()
        // .expect("Failed to read line");

        contract.set_account_id(contract.local_account_id.to_string());

    }
}