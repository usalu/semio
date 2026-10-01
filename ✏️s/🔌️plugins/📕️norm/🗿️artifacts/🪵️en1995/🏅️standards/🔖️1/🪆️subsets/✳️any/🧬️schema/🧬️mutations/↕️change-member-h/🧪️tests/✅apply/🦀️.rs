//! 🧪️ Committed vector `↕️change-member-h` / `✅apply`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{En1995Diff, En1995Mutation, En1995Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/↕️change-member-h/✅apply/🎯️outcome/🔣️.json");
fn before() -> En1995Snapshot { pack::json::from_json_str(BEFORE).expect("before") }
fn mutation() -> En1995Mutation { pack::json::from_json_str(MUTATION).expect("mutation") }
fn apply(mutation: &En1995Mutation, base: &En1995Snapshot) -> En1995Snapshot {
    let raised = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "change-member-h raised {:?}", raised.messages());
    <En1995Diff as protocol::MutationDiff<En1995Snapshot>>::apply(raised.diff(), base).expect("apply")
}
#[test]
fn mutation_is_the_canonical_wire() {
    let _: En1995Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<En1995Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "change-member-h must move the document");
    assert_eq!(after, pack::json::from_json_str::<En1995Snapshot>(AFTER).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "change-member-h changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
