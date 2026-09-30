//! 🧪️ Committed vector `🗂️rename-product-group` / `✏️to-panel`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗂️rename-product-group/✏️to-panel/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗂️rename-product-group/✏️to-panel/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗂️rename-product-group/✏️to-panel/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗂️rename-product-group/✏️to-panel/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗂️rename-product-group/✏️to-panel/🎯️outcome/🔣️.json");
fn before() -> Iso16757Snapshot { pack::json::from_json_str(BEFORE).expect("before") }
fn mutation() -> Iso16757Mutation { pack::json::from_json_str(MUTATION).expect("mutation") }
fn apply(mutation: &Iso16757Mutation, base: &Iso16757Snapshot) -> Iso16757Snapshot {
    let raised = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "rename-product-group raised {:?}", raised.messages());
    <Iso16757Diff as protocol::MutationDiff<Iso16757Snapshot>>::apply(raised.diff(), base).expect("apply")
}
#[test]
fn mutation_is_the_canonical_wire() {
    let _: Iso16757Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<Iso16757Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "rename-product-group must move the document");
    assert_eq!(after, pack::json::from_json_str::<Iso16757Snapshot>(AFTER).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "rename-product-group changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
