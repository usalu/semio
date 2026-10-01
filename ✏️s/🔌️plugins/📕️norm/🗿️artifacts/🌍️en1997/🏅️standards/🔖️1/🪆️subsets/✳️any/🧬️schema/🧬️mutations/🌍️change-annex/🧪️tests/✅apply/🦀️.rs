//! 🧪️ Committed vector `🌍️change-annex` / `✅apply`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{En1997Diff, En1997Mutation, En1997Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/✅apply/🎯️outcome/🔣️.json");
fn before() -> En1997Snapshot { pack::json::from_json_str(BEFORE).expect("before") }
fn mutation() -> En1997Mutation { pack::json::from_json_str(MUTATION).expect("mutation") }
fn apply(mutation: &En1997Mutation, base: &En1997Snapshot) -> En1997Snapshot {
    let raised = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "change-annex raised {:?}", raised.messages());
    <En1997Diff as protocol::MutationDiff<En1997Snapshot>>::apply(raised.diff(), base).expect("apply")
}
#[test]
fn mutation_is_the_canonical_wire() {
    let _: En1997Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<En1997Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "change-annex must move the document");
    assert_eq!(after, pack::json::from_json_str::<En1997Snapshot>(AFTER).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "change-annex changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
