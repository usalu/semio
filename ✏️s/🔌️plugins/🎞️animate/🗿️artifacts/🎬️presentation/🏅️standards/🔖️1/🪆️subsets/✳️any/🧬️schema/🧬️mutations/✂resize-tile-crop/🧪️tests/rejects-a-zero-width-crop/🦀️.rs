//! 🧪️ `resize-tile-crop` fixture — `rejects-a-zero-width-crop`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `PresentationSnapshot` keeps its `(source, tiles)` in the composed `s.stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! ✂️ `resize-tile-crop` guards in a strict order: target lookup, then finiteness, then POSITIVE
//! extent, and only then the unchanged-value warning. This case pins the third guard — the tile
//! really is in the seeded deck and the committed crop's numbers really are finite, so a zero width
//! is the only thing wrong, and it is FATAL: a degenerate crop rect is a geometry invariant breach,
//! not a miss.

use crate::artifacts::presentation::mutations::{apply_presentation_mutation, inverse_presentation_mutation, PresentationMutation};
use crate::artifacts::presentation::{cache_presentation_working_scene, default_figure_tile_source, FigureTileDraft, FigureTileFrame, PresentationDiff, PresentationSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> PresentationMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> PresentationSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `presentation` child resolved to a deck holding
/// the tile the payload addresses, carrying a healthy non-degenerate crop — so the rejection can
/// only come from the incoming rect, never from a missing target.
fn before() -> PresentationSnapshot {
    let snapshot: PresentationSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PresentationMutation::ResizeTileCrop(payload) = mutation() else {
        panic!("rejects-a-zero-width-crop's committed mutation must be a resize-tile-crop");
    };
    let tile = FigureTileDraft { id: payload.id, name: "Hero".into(), crop: FigureTileFrame { x: 0.25, y: 0.25, width: 0.5, height: 0.5 } };
    cache_presentation_working_scene(&snapshot.presentation.child_id, &default_figure_tile_source(), &[tile]);
    snapshot
}

/// ▶️ A rejected `resize-tile-crop` leaves the document byte-identical to the committed `after` —
/// the healthy crop in the deck is not overwritten on the way to the rejection.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_presentation_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "resize-tile-crop/rejects-a-zero-width-crop: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "a rejected recrop must not mint a new presentation handle");
}

/// 🚨️ A zero-width crop is FATAL `mutation.invariant` — a different code AND a different level from
/// the `mutation.target-missing` a bad id would raise, addressed at the tile whose geometry is at
/// stake.
#[test]
fn a_zero_width_crop_is_a_fatal_invariant() {
    let produced = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PresentationDiff::default(), "a rejecting resize-tile-crop must carry the identity diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.invariant", "a degenerate crop is an invariant breach, not a missing target");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "mutation.invariant is Fatal — no merge policy may absorb a degenerate rect");
    assert_eq!(messages[0].target, vec!["tiles".to_string(), "t-hero".to_string()], "the diagnostic addresses the collection and then the tile whose crop is degenerate");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[test]
fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PresentationDiff::default(), "resize-tile-crop/rejects-a-zero-width-crop: a Fatal outcome must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical. The committed crop
/// is deliberately finite with a zero WIDTH and a healthy HEIGHT, so it clears the finiteness guard
/// and lands squarely on the positive-extent one.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PresentationSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "resize-tile-crop/rejects-a-zero-width-crop: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "resize-tile-crop/rejects-a-zero-width-crop: committed mutation JSON is not canonical");
    let PresentationMutation::ResizeTileCrop(payload) = mutation() else {
        panic!("committed mutation must be a resize-tile-crop");
    };
    assert!(payload.new_crop.width == 0.0 && payload.new_crop.height > 0.0, "only the width is degenerate, so the positive-extent guard is the one under test");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "resize-tile-crop/rejects-a-zero-width-crop declares a rejected outcome");
    let produced = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `resize-tile-crop`'s inverse is BASE-derived from the target's CURRENT crop and never looks at
/// the payload's rect — so even a refused degenerate resize inverts to a healthy restore of the
/// crop the deck still holds.
#[test]
fn inverse_restores_the_healthy_base_crop_not_the_refused_rect() {
    let inverse = inverse_presentation_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "resize-tile-crop undoes with exactly one step once its target exists, got {inverse:?}");
    let PresentationMutation::ResizeTileCrop(undo) = &inverse[0] else {
        panic!("resize-tile-crop's inverse must be a resize-tile-crop, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "t-hero", "the inverse addresses the same tile");
    assert_eq!(undo.new_crop, FigureTileFrame { x: 0.25, y: 0.25, width: 0.5, height: 0.5 }, "the inverse carries the base crop, never the degenerate one the payload proposed");
}

/// 🪪️ The fixture is bound to `resize-tile-crop`'s own descriptor, whose address reaches all the way
/// down to the `crop` field — one segment deeper than the diagnostic's tile-level address.
#[test]
fn semantics_bind_this_fixture_to_resize_tile_crop() {
    let semantics = <PresentationMutation as protocol::SemanticMutation<PresentationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("resize", "tile-crop", "resize-tile-crop", "ResizedTileCrop"), "the fixture must be bound to resize-tile-crop's own descriptor");
    assert_eq!(<PresentationMutation as protocol::SemanticMutation<PresentationSnapshot>>::target(&mutation()), vec!["tiles".to_string(), "t-hero".to_string(), "crop".to_string()], "resize-tile-crop addresses the collection, the tile, and the crop field");
}
