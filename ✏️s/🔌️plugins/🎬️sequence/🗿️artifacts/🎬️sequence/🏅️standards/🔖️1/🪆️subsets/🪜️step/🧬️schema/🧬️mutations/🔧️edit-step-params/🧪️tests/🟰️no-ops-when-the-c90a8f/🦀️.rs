//! 🧪️ `edit-step-params` fixture — `🟰️no-ops-when-the-params-are-already-identical`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `SequenceSnapshot` keeps its steps/edges in the composed `s.stdio.semio.flow` CHILD, and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can hand-author —
//! this tree pins the guard branches, which mint nothing.
//!
//! 🔧 `edit-step-params` replaces a step's authored parameter BODY wholesale — the properties panel
//! never edits one key at a time — so its second guard is a whole-`StepParams` dictionary equality.
//! The seeded step carries the committed payload's own params, verbatim; `StepParams` is
//! `#[serde(transparent)]` over an ordered dictionary, so the committed JSON is a bare object.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::{apply_sequence_mutation, inverse_sequence_mutation, SequenceMutation};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep, SequenceWorkingScene};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> SequenceMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> SequenceSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `content` child resolved to a one-step scene whose
/// step already carries the committed payload's params dictionary — nothing about it is invented.
fn before() -> SequenceSnapshot {
    let snapshot: SequenceSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let SequenceMutation::EditStepParams(payload) = mutation() else {
        panic!("no-ops-when-the-params-are-already-identical's committed mutation must be an edit-step-params");
    };
    let step = SequenceStep { id: payload.id.clone(), kind: "log.print".into(), params: payload.params.clone(), x: 0.0, y: 0.0, slot: None, collapsed: false };
    snapshot.content.set_local_owner(std::sync::Arc::new(SequenceWorkingScene { steps: vec![step], edges: Vec::new() }));
    snapshot
}

/// ▶️ Re-submitting a step's current parameter body carries `before` to exactly the committed
/// `after`, leaving the composed content handle untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_sequence_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "edit-step-params/no-ops-when-the-params-are-already-identical: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.content.child_id, &base.content.child_id, "a params-identity edit must not re-mint the content handle");
}

/// 🔺️ The delta is exactly the committed all-null `SequenceDiff` — a whole-body edit that matches
/// the stored body must not rebuild the scene, which would churn every OTHER step's addressing too.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-step-params/no-ops-when-the-params-are-already-identical: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &SequenceDiff::default(), "a params-identity edit must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to sequence's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SequenceDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-step-params/no-ops-when-the-params-are-already-identical: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`, with the
/// params-bearing content slot never set.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SequenceDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.content.is_none(), "a params-identity edit must leave the composed content slot unset");
    let produced = <SequenceDiff as protocol::MutationDiff<SequenceSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-step-params/no-ops-when-the-params-are-already-identical: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical — `params` is a bare
/// JSON object, because `StepParams` is `#[serde(transparent)]` over its dictionary rather than a
/// wrapper with a named field.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SequenceSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-step-params/no-ops-when-the-params-are-already-identical: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-step-params/no-ops-when-the-params-are-already-identical: committed mutation JSON is not canonical");
    assert!(original.get("params").map(serde_json::Value::is_object).unwrap_or(false), "a transparent StepParams encodes as a bare object, never as a wrapper");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-step-params/no-ops-when-the-params-are-already-identical declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "unchanged parameters are a warning, not a missing-target error");
}

/// ↩️ `edit-step-params`' inverse is BASE-derived and restores the WHOLE captured body, so here it is
/// the committed payload itself — a body-identical edit is its own inverse.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_whole_captured_params_body() {
    let base = before();
    let SequenceMutation::EditStepParams(payload) = mutation() else {
        panic!("committed mutation must be an edit-step-params");
    };
    let inverse = inverse_sequence_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "edit-step-params undoes with exactly one step, got {inverse:?}");
    let SequenceMutation::EditStepParams(undo) = &inverse[0] else {
        panic!("edit-step-params' inverse must be an edit-step-params, got {:?}", inverse[0]);
    };
    assert_eq!((undo.id.as_str(), &undo.params), (payload.id.as_str(), &payload.params), "the inverse restores the captured body, which here equals the requested one");
    let restored = apply_sequence_mutation(&apply_sequence_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "edit-step-params/no-ops-when-the-params-are-already-identical: inverse did not restore the before-snapshot");
}
