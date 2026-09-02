//! 🧪️ `move-step` fixture — `no-ops-when-the-step-is-already-at-that-position`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `SequenceSnapshot` keeps its steps/edges in the composed `s.stdio.semio.flow` CHILD, and every
//! content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can hand-author —
//! this tree pins the guard branches, which mint nothing.
//!
//! 📍 `move-step` carries a FINAL-state absolute `(x, y)` and guards in three stages: target lookup,
//! finiteness, then coordinate identity. This case pins the third — the seeded step already sits at
//! the committed payload's own coordinates, which are dyadic so the `f64` equality is exact rather
//! than luck.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::{apply_sequence_mutation, inverse_sequence_mutation, SequenceMutation};
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep, SequenceWorkingScene, StepParams};

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
/// step sits at exactly the committed payload's `(x, y)`. Only the id and the coordinates matter to
/// this verb's guards.
fn before() -> SequenceSnapshot {
    let snapshot: SequenceSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let SequenceMutation::MoveStep(payload) = mutation() else {
        panic!("no-ops-when-the-step-is-already-at-that-position's committed mutation must be a move-step");
    };
    let step = SequenceStep { id: payload.id.clone(), kind: "log.print".into(), params: StepParams::default(), x: payload.x, y: payload.y, slot: None, collapsed: false };
    snapshot.content.set_local_owner(std::sync::Arc::new(SequenceWorkingScene { steps: vec![step], edges: Vec::new() }));
    snapshot
}

/// ▶️ Moving a step to the coordinates it already holds carries `before` to exactly the committed
/// `after`, leaving the composed content handle untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_sequence_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "move-step/no-ops-when-the-step-is-already-at-that-position: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.content.child_id, &base.content.child_id, "a positional no-op must not re-mint the content handle");
}

/// 🔺️ The delta is exactly the committed all-null `SequenceDiff` — reaching it also proves the
/// committed coordinates cleared the finiteness guard that sits between the lookup and the identity
/// test.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-step/no-ops-when-the-step-is-already-at-that-position: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &SequenceDiff::default(), "a positional no-op must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to sequence's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SequenceDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-step/no-ops-when-the-step-is-already-at-that-position: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`. Step order and
/// position both live in the composed content child, and this diff leaves that slot unset.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SequenceDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.content.is_none(), "a positional no-op must leave the order-and-position-bearing content slot unset");
    let produced = <SequenceDiff as protocol::MutationDiff<SequenceSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-step/no-ops-when-the-step-is-already-at-that-position: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical, with dyadic
/// coordinates so the `f64` round trip is exact.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SequenceSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-step/no-ops-when-the-step-is-already-at-that-position: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-step/no-ops-when-the-step-is-already-at-that-position: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op` — not the
/// Fatal `mutation.invariant` this same verb raises for a non-finite coordinate.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-step/no-ops-when-the-step-is-already-at-that-position declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an already-there move is a warning, never the Fatal a non-finite position raises");
}

/// ↩️ `move-step`'s inverse is BASE-derived: the coordinates the step currently holds. Here those
/// ARE the requested ones, so the move is its own inverse and replaying it changes nothing.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_base_position() {
    let base = before();
    let inverse = inverse_sequence_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "move-step undoes with exactly one step, got {inverse:?}");
    let SequenceMutation::MoveStep(undo) = &inverse[0] else {
        panic!("move-step's inverse must be a move-step, got {:?}", inverse[0]);
    };
    assert_eq!((undo.id.as_str(), undo.x, undo.y), ("step-log", 280.0, 0.0), "the inverse carries the base coordinates, which here equal the requested ones");
    let restored = apply_sequence_mutation(&apply_sequence_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "move-step/no-ops-when-the-step-is-already-at-that-position: inverse did not restore the before-snapshot");
}
