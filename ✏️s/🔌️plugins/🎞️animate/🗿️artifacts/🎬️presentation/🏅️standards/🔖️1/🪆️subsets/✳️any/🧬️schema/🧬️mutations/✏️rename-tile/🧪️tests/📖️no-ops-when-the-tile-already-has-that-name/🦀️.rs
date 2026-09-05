//! 🧪️ `rename-tile` fixture — `📖️no-ops-when-the-tile-already-has-that-name`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `PresentationSnapshot` keeps its `(source, tiles)` in the composed `s.stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! ✏️ `rename-tile` touches exactly one scalar (`tiles[id].name`) and its second guard is a plain
//! string equality on that scalar. The seeded deck's tile carries the committed payload's own
//! `new_name`, so the rename is a genuine identity — and the diff must stay empty rather than
//! re-minting a deck handle for a name that did not move.

use crate::artifacts::presentation::mutations::{apply_presentation_mutation, inverse_presentation_mutation, PresentationMutation};
use crate::artifacts::presentation::{cache_presentation_working_scene, default_figure_tile_source, FigureTileDraft, FigureTileFrame, PresentationDiff, PresentationSnapshot};

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

/// 🌱 The committed `⬅️before`, with its composed `presentation` child resolved to a deck whose one
/// tile already carries the committed payload's `new_name` — the identity the second guard catches.
fn before() -> PresentationSnapshot {
    let snapshot: PresentationSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let PresentationMutation::RenameTile(payload) = mutation() else {
        panic!("no-ops-when-the-tile-already-has-that-name's committed mutation must be a rename-tile");
    };
    let tile = FigureTileDraft { id: payload.id, name: payload.new_name, crop: FigureTileFrame { x: 0.25, y: 0.25, width: 0.5, height: 0.5 } };
    cache_presentation_working_scene(&snapshot.presentation.child_id, &default_figure_tile_source(), &[tile]);
    snapshot
}

/// ▶️ Renaming a tile to the name it already has carries `before` to exactly the committed `after`,
/// leaving the composed deck handle untouched.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_presentation_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "rename-tile/no-ops-when-the-tile-already-has-that-name: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "an identity rename must not re-mint the presentation handle");
}

/// 🔺️ The delta is exactly the committed all-null `PresentationDiff` — the guard returns after the target
/// lookup succeeded but before the name patch, so no deck is rebuilt for a non-change.
#[test]
fn produces_committed_diff() {
    let outcome = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-tile/no-ops-when-the-tile-already-has-that-name: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &PresentationDiff::default(), "an identity rename must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to presentation's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: PresentationDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-tile/no-ops-when-the-tile-already-has-that-name: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`, with the
/// tile-bearing `presentation` slot never set.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: PresentationDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.presentation.is_none(), "an identity rename must leave the composed deck slot unset");
    let produced = <PresentationDiff as protocol::MutationDiff<PresentationSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-tile/no-ops-when-the-tile-already-has-that-name: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PresentationSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-tile/no-ops-when-the-tile-already-has-that-name: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-tile/no-ops-when-the-tile-already-has-that-name: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: `applied`, with one untargeted Warning `mutation.no-op` — a
/// presentation tile with an unchanged name is a warning, never the Error a missing id would raise.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-tile/no-ops-when-the-tile-already-has-that-name declares an applied outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "an unchanged name is a warning, not a missing-target error");
    assert!(messages[0].target.is_empty(), "rename-tile's no-op warning carries no target address");
}

/// ↩️ `rename-tile`'s inverse is BASE-derived: it restores the name the tile currently carries.
/// Here that IS the requested name, so the rename is its own inverse and replaying it changes
/// nothing.
#[test]
fn inverse_restores_the_base_name_and_is_its_own_inverse_here() {
    let base = before();
    let inverse = inverse_presentation_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "rename-tile undoes with exactly one step, got {inverse:?}");
    let PresentationMutation::RenameTile(undo) = &inverse[0] else {
        panic!("rename-tile's inverse must be a rename-tile, got {:?}", inverse[0]);
    };
    assert_eq!((undo.id.as_str(), undo.new_name.as_str()), ("t-hero", "Hero"), "the inverse restores the captured name, which here equals the requested one");
    let restored = apply_presentation_mutation(&apply_presentation_mutation(&base, &mutation()).expect("forward applies"), &inverse[0]).expect("inverse step applies");
    assert_eq!(restored, base, "rename-tile/no-ops-when-the-tile-already-has-that-name: inverse did not restore the before-snapshot");
}
