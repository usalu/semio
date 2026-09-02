//! 🧪️ `resize-source-frame` fixture — `no-ops-when-the-frame-is-already-identical`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `PresentationSnapshot` keeps its `(source, tiles)` in the composed `s.stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! 🔲 `resize-source-frame` carves ONE field out of the source facet — `source.frame` — which is why
//! it exists alongside the whole-value `replace-source`. Its guards run finiteness, then positive
//! extent, then value identity; this case pins the LAST one, so the committed frame is deliberately
//! finite and positive and the only thing "wrong" with it is that it is already in place. The
//! seeded source is the plugin's own `default_figure_tile_source()` with that frame swapped in, so
//! every other source field stays real.

use crate::artifacts::presentation::mutations::{apply_presentation_mutation, inverse_presentation_mutation, PresentationMutation};
use crate::artifacts::presentation::{cache_presentation_working_scene, default_figure_tile_source, PresentationDiff, PresentationSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> PresentationMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> PresentationSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `presentation` child resolved to the plugin's own
/// default source carrying the committed payload's `newFrame` — the identity the third guard sees.
fn before() -> PresentationSnapshot {
    let snapshot: PresentationSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PresentationMutation::ResizeSourceFrame(payload) = mutation() else {
        panic!("no-ops-when-the-frame-is-already-identical's committed mutation must be a resize-source-frame");
    };
    let mut source = default_figure_tile_source();
    source.frame = payload.new_frame;
    cache_presentation_working_scene(&snapshot.presentation.child_id, &source, &[]);
    snapshot
}

/// ▶️ Resizing the source frame to the rect it already has carries `before` to exactly the committed
/// `after`, leaving the composed deck handle untouched.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_presentation_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "resize-source-frame/no-ops-when-the-frame-is-already-identical: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "a frame-identity resize must not re-mint the presentation handle");
}

/// 🔺️ The delta is exactly the committed all-null `PresentationDiff` — the value-identity guard sits
/// AFTER the two geometry invariants, so reaching an empty diff here proves the committed frame was
/// accepted as valid and then recognised as unchanged, not rejected.
#[test]
fn produces_committed_diff() {
    let outcome = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "resize-source-frame/no-ops-when-the-frame-is-already-identical: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &PresentationDiff::default(), "a frame-identity resize must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to presentation's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: PresentationDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "resize-source-frame/no-ops-when-the-frame-is-already-identical: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`, with the deck
/// slot never set.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: PresentationDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.presentation.is_none(), "a frame-identity resize must leave the composed deck slot unset");
    let produced = <PresentationDiff as protocol::MutationDiff<PresentationSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "resize-source-frame/no-ops-when-the-frame-is-already-identical: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. The committed frame
/// is finite with strictly positive extents, so it clears both invariant guards on the way to the
/// identity branch.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PresentationSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "resize-source-frame/no-ops-when-the-frame-is-already-identical: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "resize-source-frame/no-ops-when-the-frame-is-already-identical: committed mutation JSON is not canonical");
    let PresentationMutation::ResizeSourceFrame(payload) = mutation() else {
        panic!("committed mutation must be a resize-source-frame");
    };
    assert!(payload.new_frame.width > 0.0 && payload.new_frame.height > 0.0 && payload.new_frame.x.is_finite(), "the committed frame must clear both geometry invariants so the identity guard is the one under test");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op` — not the
/// Fatal `mutation.invariant` this same verb raises for a degenerate rect.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "resize-source-frame/no-ops-when-the-frame-is-already-identical declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an unchanged frame is a warning, never the Fatal a degenerate rect raises");
}

/// ↩️ `resize-source-frame`'s inverse ignores its payload and rebuilds from the BASE source's frame,
/// so here it is the same rect — a value-identical resize is its own inverse.
#[test]
fn inverse_restores_the_base_frame() {
    let base = before();
    let PresentationMutation::ResizeSourceFrame(payload) = mutation() else {
        panic!("committed mutation must be a resize-source-frame");
    };
    let inverse = inverse_presentation_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "resize-source-frame always undoes with exactly one step, got {inverse:?}");
    let PresentationMutation::ResizeSourceFrame(undo) = &inverse[0] else {
        panic!("resize-source-frame's inverse must be a resize-source-frame, got {:?}", inverse[0]);
    };
    assert_eq!(undo.new_frame, payload.new_frame, "the inverse restores the captured base frame, which here equals the requested one");
    let restored = apply_presentation_mutation(&apply_presentation_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "resize-source-frame/no-ops-when-the-frame-is-already-identical: inverse did not restore the before-snapshot");
}
