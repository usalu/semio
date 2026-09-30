//! 🧪️ Committed vector `🧱change-plate-thickness` / `✏️to-0-0125`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{En1999Diff, En1999Mutation, En1999Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-plate-thickness/✏️to-0-0125/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-plate-thickness/✏️to-0-0125/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-plate-thickness/✏️to-0-0125/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-plate-thickness/✏️to-0-0125/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-plate-thickness/✏️to-0-0125/🎯️outcome/🔣️.json");
fn before() -> En1999Snapshot { pack::json::from_json_str(BEFORE).expect("before") }
fn mutation() -> En1999Mutation { pack::json::from_json_str(MUTATION).expect("mutation") }
fn apply(mutation: &En1999Mutation, base: &En1999Snapshot) -> En1999Snapshot {
    let raised = <En1999Mutation as protocol::Mutation<En1999Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "change-plate-thickness raised {:?}", raised.messages());
    <En1999Diff as protocol::MutationDiff<En1999Snapshot>>::apply(raised.diff(), base).expect("apply")
}
#[test]
fn mutation_is_the_canonical_wire() {
    let _: En1999Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <En1999Mutation as protocol::Mutation<En1999Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<En1999Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "change-plate-thickness must move the document");
    assert_eq!(after, pack::json::from_json_str::<En1999Snapshot>(AFTER).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <En1999Mutation as protocol::Mutation<En1999Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "change-plate-thickness changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
