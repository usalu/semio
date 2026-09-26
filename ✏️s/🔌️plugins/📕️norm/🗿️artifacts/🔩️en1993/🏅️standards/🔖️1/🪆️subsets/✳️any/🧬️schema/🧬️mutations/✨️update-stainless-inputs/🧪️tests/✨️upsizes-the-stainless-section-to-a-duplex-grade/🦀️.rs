//! 🧪️ Hierarchical fixture triad `✨️update-stainless-inputs` / `✨️upsizes-the-stainless-section-to-a-duplex-grade`.
use crate::{En1993Diff, En1993Mutation, En1993Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✨️update-stainless-inputs/✨️upsizes-the-stainless-section-to-a-duplex-grade/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✨️update-stainless-inputs/✨️upsizes-the-stainless-section-to-a-duplex-grade/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✨️update-stainless-inputs/✨️upsizes-the-stainless-section-to-a-duplex-grade/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✨️update-stainless-inputs/✨️upsizes-the-stainless-section-to-a-duplex-grade/🎯️outcome/🔣️.json");
fn before() -> En1993Snapshot { serde_json::from_str(BEFORE).expect("before") }
fn expected_after() -> En1993Snapshot { serde_json::from_str(AFTER).expect("after") }
fn mutation() -> En1993Mutation { serde_json::from_str(MUTATION).expect("mutation") }
fn applied() -> En1993Snapshot {
    let base = before();
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation(), &base);
    <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(raised.diff(), &base).expect("apply")
}
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = applied();
    assert!(snapshot.materials.iter().any(|m| m.id == "mat-s460"));
    assert_eq!(snapshot, expected_after());
}
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty());
    let mut snapshot = applied();
    for step in &inverse {
        let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(step, &snapshot);
        snapshot = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse");
    }
    assert_eq!(snapshot, base);
}
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome");
    assert_eq!(outcome["status"], "applied");
}
