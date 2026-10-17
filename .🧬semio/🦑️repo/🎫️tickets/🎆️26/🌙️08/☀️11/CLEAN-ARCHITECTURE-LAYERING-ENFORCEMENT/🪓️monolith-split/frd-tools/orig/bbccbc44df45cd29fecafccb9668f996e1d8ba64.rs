//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_counter_leaf_contract() {
    super::super::super::assert_counter_leaf_descriptor::<AddCounterFourTimes>(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️testing/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🔣️.json")).expect("fixture").as_str());
}
#[test]
fn plan_nests_two_twice_plans() {
    let base = 0;
    let mut planner = Planner::new(&base);
    AddCounterFourTimes { delta: 2 }.plan(&base, &mut planner).unwrap();
    assert_eq!(planner.steps().len(), 4);
}
