//! 🧫️ Committed vector `🚫️remove-element` / `🔬️middle-row`: removes the MIDDLE row of a three-row collection; the canonical wire reaches the committed after-snapshot and diff, its own inverse restores the before-snapshot with the row back at its original position, and the inverse diffs sum to the negative diff.
use crate::diff::Din4108Diff;
use crate::{Din4108Mutation, Din4108Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚫️remove-element/🔬️middle-row/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚫️remove-element/🔬️middle-row/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚫️remove-element/🔬️middle-row/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚫️remove-element/🔬️middle-row/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚫️remove-element/🔬️middle-row/🎯️outcome/🔣️.json");
fn parse<T: semio_framework_value::FromValue>(text: &str) -> T {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed json")
}
fn apply(mutation: &Din4108Mutation, base: &Din4108Snapshot) -> Din4108Snapshot {
    let raised = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "remove-element raised {:?}", raised.messages());
    protocol::apply_diff(raised.diff(), base).expect("apply")
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base: Din4108Snapshot = parse(BEFORE);
    let mutation: Din4108Mutation = parse(MUTATION);
    let raised = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation, &base);
    assert_eq!(*raised.diff(), parse::<Din4108Diff>(DIFF));
    assert_eq!(apply(&mutation, &base), parse::<Din4108Snapshot>(AFTER));
}
#[test]
fn inverse_restores_before() {
    let base: Din4108Snapshot = parse(BEFORE);
    let mutation: Din4108Mutation = parse(MUTATION);
    let inverse = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    assert!(!inverse.is_empty(), "remove-element changes the document, so its inverse must not be empty");
    let restored = inverse.iter().rev().fold(apply(&mutation, &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&parse::<Din4108Mutation>(MUTATION), &parse::<Din4108Snapshot>(BEFORE)).await;
}
