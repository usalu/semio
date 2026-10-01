//! 🧪️ Committed vector `🧯️change-insulation-thickness-m` / `✅apply`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{En1994Diff, En1994Mutation, En1994Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✅apply/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✅apply/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✅apply/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✅apply/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯️change-insulation-thickness-m/✅apply/🎯️outcome/🔣️.json");
fn before() -> En1994Snapshot { pack::json::from_json_str(BEFORE).expect("before") }
fn mutation() -> En1994Mutation { pack::json::from_json_str(MUTATION).expect("mutation") }
fn apply(mutation: &En1994Mutation, base: &En1994Snapshot) -> En1994Snapshot {
    let raised = <En1994Mutation as protocol::Mutation<En1994Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "change-insulation-thickness-m raised {:?}", raised.messages());
    <En1994Diff as protocol::MutationDiff<En1994Snapshot>>::apply(raised.diff(), base).expect("apply")
}
#[test]
fn mutation_is_the_canonical_wire() {
    let _: En1994Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <En1994Mutation as protocol::Mutation<En1994Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<En1994Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "change-insulation-thickness-m must move the document");
    assert_eq!(after, pack::json::from_json_str::<En1994Snapshot>(AFTER).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <En1994Mutation as protocol::Mutation<En1994Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "change-insulation-thickness-m changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
