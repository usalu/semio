//! 🧪️ Committed vector `🧱change-concrete-f-ck` / `✅apply`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{En1992Diff, En1992Mutation, En1992Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-concrete-f-ck/✅apply/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-concrete-f-ck/✅apply/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-concrete-f-ck/✅apply/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-concrete-f-ck/✅apply/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-concrete-f-ck/✅apply/🎯️outcome/🔣️.json");
fn before() -> En1992Snapshot { pack::json::from_json_str(BEFORE).expect("before") }
fn mutation() -> En1992Mutation { pack::json::from_json_str(MUTATION).expect("mutation") }
fn apply(mutation: &En1992Mutation, base: &En1992Snapshot) -> En1992Snapshot {
    let raised = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "change-concrete-f-ck raised {:?}", raised.messages());
    <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(raised.diff(), base).expect("apply")
}
#[test]
fn mutation_is_the_canonical_wire() {
    let _: En1992Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<En1992Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "change-concrete-f-ck must move the document");
    assert_eq!(after, pack::json::from_json_str::<En1992Snapshot>(AFTER).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "change-concrete-f-ck changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
