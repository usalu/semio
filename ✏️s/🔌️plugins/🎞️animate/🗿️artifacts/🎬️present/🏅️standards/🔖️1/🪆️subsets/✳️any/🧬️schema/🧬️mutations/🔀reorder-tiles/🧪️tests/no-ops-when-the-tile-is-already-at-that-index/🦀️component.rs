//! 🧪️ `reorder-tiles` fixture — `no-ops-when-the-tile-is-already-at-that-index`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `PresentSnapshot` keeps its `(source, tiles)` in the composed `s.stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! 🔀 `reorder-tiles` is the one present verb that carries ORDER, and its guard does the comparison
//! on the list with the tile ALREADY REMOVED (`tiles.remove(from)` then `to_index.min(tiles.len())`).
//! The seeded deck holds a single tile, so index 0 is both its current position and the only
//! reachable slot — the exact arithmetic that guard exists for. Crucially the resulting diff sets no
//! ordering-bearing field at all: `PresentDiff` expresses order only through a re-minted
//! `presentation` handle, and this case must not mint one.

use crate::artifacts::present::mutations::{apply_present_mutation, inverse_present_mutation, PresentMutation};
use crate::artifacts::present::{cache_present_working_scene, default_figure_tile_source, FigureTileDraft, FigureTileFrame, PresentDiff, PresentSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> PresentMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> PresentSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `presentation` child resolved to a single-tile
/// deck whose only tile is the id the payload names. Only the id and the list length matter here.
fn before() -> PresentSnapshot {
    let snapshot: PresentSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PresentMutation::ReorderTiles(payload) = mutation() else {
        panic!("no-ops-when-the-tile-is-already-at-that-index's committed mutation must be a reorder-tiles");
    };
    let tile = FigureTileDraft { id: payload.id.clone(), name: "Hero".into(), crop: FigureTileFrame { x: 0.25, y: 0.25, width: 0.5, height: 0.5 } };
    cache_present_working_scene(&snapshot.presentation.child_id, &default_figure_tile_source(), &[tile]);
    snapshot
}

/// ▶️ Reordering a tile onto the slot it already occupies carries `before` to exactly the committed
/// `after`, leaving the composed deck handle untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_present_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "reorder-tiles/no-ops-when-the-tile-is-already-at-that-index: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "a positional no-op must not re-mint the presentation handle");
}

/// 🔺️ The delta is exactly the committed all-null `PresentDiff`: the guard fires after the removal
/// arithmetic but before the reinsertion, so no reordered deck is ever built.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-tiles/no-ops-when-the-tile-is-already-at-that-index: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(outcome.diff(), &PresentDiff::default(), "a positional no-op must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to present's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PresentDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-tiles/no-ops-when-the-tile-is-already-at-that-index: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`. Order lives in
/// the composed deck, and this diff leaves that slot unset — the ordering-pinning assertion this
/// verb specifically needs.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PresentDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.presentation.is_none(), "a positional no-op must leave the order-bearing presentation slot unset");
    let produced = <PresentDiff as protocol::MutationDiff<PresentSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-tiles/no-ops-when-the-tile-is-already-at-that-index: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PresentSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-tiles/no-ops-when-the-tile-is-already-at-that-index: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-tiles/no-ops-when-the-tile-is-already-at-that-index: committed mutation JSON is not canonical");
    assert_eq!(original.get("ReorderTiles").and_then(|payload| payload.get("toIndex")).and_then(serde_json::Value::as_u64), Some(0), "the landing slot is addressed by id plus a final-state index");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-tiles/no-ops-when-the-tile-is-already-at-that-index declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an already-there reorder is a warning, not a missing-target error");
}

/// ↩️ `reorder-tiles`' inverse is BASE-derived: it moves the tile back to the index it currently
/// holds. Here that index IS the requested one, so replaying it leaves the deck exactly as it was.
#[semio_framework_async_macros::async_test]
async fn inverse_moves_the_tile_back_to_its_base_index() {
    let base = before();
    let inverse = inverse_present_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "reorder-tiles undoes with exactly one step, got {inverse:?}");
    let PresentMutation::ReorderTiles(undo) = &inverse[0] else {
        panic!("reorder-tiles' inverse must be a reorder-tiles, got {:?}", inverse[0]);
    };
    assert_eq!((undo.id.as_str(), undo.to_index), ("t-hero", 0), "the inverse restores the base index of the tile the payload named");
    let restored = apply_present_mutation(&apply_present_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "reorder-tiles/no-ops-when-the-tile-is-already-at-that-index: inverse did not restore the before-snapshot");
}
