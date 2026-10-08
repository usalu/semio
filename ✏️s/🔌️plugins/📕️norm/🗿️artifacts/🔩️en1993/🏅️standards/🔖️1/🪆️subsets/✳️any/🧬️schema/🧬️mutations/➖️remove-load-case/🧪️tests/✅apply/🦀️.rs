//! 🧪️ Committed vector `➖️remove-load-case` / `✅apply`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{En1993Diff, En1993Mutation, En1993Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-load-case/✅apply/🎯️outcome/🔣️.json");
fn before() -> En1993Snapshot { semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before") }
fn mutation() -> En1993Mutation { semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation") }
fn apply(mutation: &En1993Mutation, base: &En1993Snapshot) -> En1993Snapshot {
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "remove-load-case raised {:?}", raised.messages());
    protocol::apply_diff(raised.diff(), base).expect("apply")
}
#[test]
fn mutation_is_the_canonical_wire() {
    let _: En1993Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), semio_framework_pack_json::from_json_str::<En1993Diff>(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "remove-load-case must move the document");
    assert_eq!(after, semio_framework_pack_json::from_json_str::<En1993Snapshot>(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert!(!inverse.is_empty(), "remove-load-case changes the document, so its inverse must not be empty");
    let restored = inverse.iter().rev().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
