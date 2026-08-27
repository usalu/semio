//! 🧪️ `change-step-collapsed` fixture — `no-ops-when-the-step-is-already-collapsed`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `SequenceSnapshot` keeps its steps/edges in the composed `s.stdio.semio.flow` CHILD, and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can hand-author —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🗂️ `change-step-collapsed` is the narrowest verb in this vocabulary — one boolean on one step —
//! and it is a SETTER, not a toggle: the payload states the desired value outright, so re-sending
//! the value a step already holds is a no-op rather than an inversion. That distinction is exactly
//! what this case pins; a toggle would have flipped the step to expanded here.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::{apply_sequence_mutation, inverse_sequence_mutation, SequenceMutation};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep, SequenceWorkingScene, StepParams};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> SequenceMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> SequenceSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `content` child resolved to a one-step scene whose
/// step already carries the committed payload's `collapsed` value.
fn before() -> SequenceSnapshot {
    let snapshot: SequenceSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let SequenceMutation::ChangeStepCollapsed(payload) = mutation() else {
        panic!("no-ops-when-the-step-is-already-collapsed's committed mutation must be a change-step-collapsed");
    };
    let step = SequenceStep { id: payload.id.clone(), kind: "log.print".into(), params: StepParams::default(), x: 0.0, y: 0.0, slot: None, collapsed: payload.collapsed };
    snapshot.content.set_local_owner(std::sync::Arc::new(SequenceWorkingScene { steps: vec![step], edges: Vec::new() }));
    snapshot
}

/// ▶️ Collapsing an already-collapsed step carries `before` to exactly the committed `after`,
/// leaving the composed content handle untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_sequence_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "change-step-collapsed/no-ops-when-the-step-is-already-collapsed: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.content.child_id, &base.content.child_id, "a boolean-identity change must not re-mint the content handle");
}

/// 🔺️ The delta is exactly the committed all-null `SequenceDiff`. This is the cheapest possible
/// edit in the vocabulary and it still costs a WHOLE re-minted content handle when it does change
/// anything — which is precisely why the identity guard must return before building one.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-step-collapsed/no-ops-when-the-step-is-already-collapsed: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(outcome.diff(), &SequenceDiff::default(), "a boolean-identity change must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to sequence's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SequenceDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-step-collapsed/no-ops-when-the-step-is-already-collapsed: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`, with the
/// content slot never set.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SequenceDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.content.is_none(), "a boolean-identity change must leave the composed content slot unset");
    let produced = <SequenceDiff as protocol::MutationDiff<SequenceSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-step-collapsed/no-ops-when-the-step-is-already-collapsed: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical — `collapsed` is a
/// real boolean in the payload, the value being SET, not a flag meaning "flip".
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SequenceSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-step-collapsed/no-ops-when-the-step-is-already-collapsed: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-step-collapsed/no-ops-when-the-step-is-already-collapsed: committed mutation JSON is not canonical");
    assert_eq!(original.get("collapsed").and_then(serde_json::Value::as_bool), Some(true), "the payload states the desired value outright");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-step-collapsed/no-ops-when-the-step-is-already-collapsed declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an unchanged collapsed flag is a warning, not a missing-target error");
}

/// ↩️ `change-step-collapsed`'s inverse is BASE-derived: it re-states the value the step currently
/// holds. Because the verb is a setter, that means the inverse of a no-op is the SAME no-op — a
/// toggle-shaped verb would instead have inverted here.
#[semio_framework_async_macros::async_test]
async fn inverse_restates_the_base_value_rather_than_toggling() {
    let base = before();
    let inverse = inverse_sequence_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "change-step-collapsed undoes with exactly one step, got {inverse:?}");
    let SequenceMutation::ChangeStepCollapsed(undo) = &inverse[0] else {
        panic!("change-step-collapsed's inverse must be a change-step-collapsed, got {:?}", inverse[0]);
    };
    assert_eq!((undo.id.as_str(), undo.collapsed), ("step-log", true), "the inverse restates the base value, it does not flip it");
    let restored = apply_sequence_mutation(&apply_sequence_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "change-step-collapsed/no-ops-when-the-step-is-already-collapsed: inverse did not restore the before-snapshot");
}
