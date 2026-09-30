//! 🧪️ Committed vector `🚮️remove-geometry` / `➖️geom-valve-50`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮️remove-geometry/➖️geom-valve-50/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮️remove-geometry/➖️geom-valve-50/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮️remove-geometry/➖️geom-valve-50/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮️remove-geometry/➖️geom-valve-50/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮️remove-geometry/➖️geom-valve-50/🎯️outcome/🔣️.json");
fn before() -> Vdi3805Snapshot { pack::json::from_json_str(BEFORE).expect("before") }
fn mutation() -> Vdi3805Mutation { pack::json::from_json_str(MUTATION).expect("mutation") }
fn apply(mutation: &Vdi3805Mutation, base: &Vdi3805Snapshot) -> Vdi3805Snapshot {
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "remove-geometry raised {:?}", raised.messages());
    <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(raised.diff(), base).expect("apply")
}
#[test]
fn mutation_is_the_canonical_wire() {
    let _: Vdi3805Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<Vdi3805Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "remove-geometry must move the document");
    assert_eq!(after, pack::json::from_json_str::<Vdi3805Snapshot>(AFTER).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "remove-geometry changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
