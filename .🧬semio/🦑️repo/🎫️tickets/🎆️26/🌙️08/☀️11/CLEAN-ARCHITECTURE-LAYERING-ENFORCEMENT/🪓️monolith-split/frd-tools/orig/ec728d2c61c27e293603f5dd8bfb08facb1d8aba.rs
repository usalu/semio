//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_counter_leaf_contract() {
    super::super::super::assert_counter_leaf_descriptor::<AddCounter>(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️testing/🧬️mutation-laws/🧬️mutations/➕️add-counter/🔣️.json")).expect("fixture").as_str());
}
