//! 🧫️ Committed vector `➖remove-permanent` / `🔬️middle-row`: removes the MIDDLE row of a three-row collection; the canonical wire reaches the committed after-snapshot and diff, its own inverse restores the before-snapshot with the row back at its original position, and the inverse diffs sum to the negative diff.
use crate::diff::En1990Diff;
use crate::{En1990Mutation, En1990Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/🔬️middle-row/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/🔬️middle-row/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/🔬️middle-row/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/🔬️middle-row/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/🔬️middle-row/🎯️outcome/🔣️.json");
fn parse<T: semio_framework_value::FromValue>(text: &str) -> T {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed json")
}
fn apply(mutation: &En1990Mutation, base: &En1990Snapshot) -> En1990Snapshot {
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "remove-permanent raised {:?}", raised.messages());
    protocol::apply_diff(raised.diff(), base).expect("apply")
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base: En1990Snapshot = parse(BEFORE);
    let mutation: En1990Mutation = parse(MUTATION);
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation, &base);
    assert_eq!(*raised.diff(), parse::<En1990Diff>(DIFF));
    assert_eq!(apply(&mutation, &base), parse::<En1990Snapshot>(AFTER));
}
#[test]
fn inverse_restores_before() {
    let base: En1990Snapshot = parse(BEFORE);
    let mutation: En1990Mutation = parse(MUTATION);
    let inverse = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    assert!(!inverse.is_empty(), "remove-permanent changes the document, so its inverse must not be empty");
    let restored = inverse.iter().rev().fold(apply(&mutation, &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&parse::<En1990Mutation>(MUTATION), &parse::<En1990Snapshot>(BEFORE)).await;
}
