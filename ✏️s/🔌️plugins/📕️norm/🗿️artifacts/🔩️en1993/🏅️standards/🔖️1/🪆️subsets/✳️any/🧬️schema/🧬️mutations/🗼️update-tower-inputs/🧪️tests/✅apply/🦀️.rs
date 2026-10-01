//! 🧪️ Committed vector `🗼️update-tower-inputs` / `✅apply`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{En1993Diff, En1993Mutation, En1993Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗼️update-tower-inputs/✅apply/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗼️update-tower-inputs/✅apply/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗼️update-tower-inputs/✅apply/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗼️update-tower-inputs/✅apply/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗼️update-tower-inputs/✅apply/🎯️outcome/🔣️.json");
fn before() -> En1993Snapshot { pack::json::from_json_str(BEFORE).expect("before") }
fn mutation() -> En1993Mutation { pack::json::from_json_str(MUTATION).expect("mutation") }
fn apply(mutation: &En1993Mutation, base: &En1993Snapshot) -> En1993Snapshot {
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "update-tower-inputs raised {:?}", raised.messages());
    <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(raised.diff(), base).expect("apply")
}
#[test]
fn mutation_is_the_canonical_wire() {
    let _: En1993Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<En1993Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "update-tower-inputs must move the document");
    assert_eq!(after, pack::json::from_json_str::<En1993Snapshot>(AFTER).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "update-tower-inputs changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
