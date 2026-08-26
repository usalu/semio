//! 🧪️ `change-unit` fixture — `switches-the-masonry-unit-from-clay-to-calcium-silicate`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-unit` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1996Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1996Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1996Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-unit` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Switching the unit material from clay to calcium silicate rewrites `unit` alone. It raises the EN 1996-1-2
/// required thickness by the ×1.1 factor and tightens the EN 1996-2 Annex B mortar admissibility, but both
/// are computed downstream, so the declared mortar class must ride through unchanged.
#[semio_framework_async_macros::async_test]
fn switches_the_masonry_unit_from_clay_to_calcium_silicate() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-unit applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.unit, "calcium_silicate", "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: unit must read "calcium_silicate" once the change lands");
    assert_eq!(
        applied.mortar,
        before().mortar,
        "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the mortar class is what the exposure/unit admissibility check reads and must not be silently upgraded to keep the combination admissible"
    );
}

/// ↩️ `change-unit`'s inverse reads the OLD "clay" out of BASE, so replaying it puts the clay unit back on
/// `unit`.
#[semio_framework_async_macros::async_test]
fn returning_to_clay_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-unit applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the inverse of one change-unit is exactly one change-unit back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-unit inverse step applies");
    }
    assert_eq!(snapshot.unit, base.unit, "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the inverse must put the clay unit back on `unit`");
    assert_eq!(snapshot, base, "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-unit` payload are already canonical: decode → encode is
/// a fixed point, so `newUnit`, a plain JSON string (the field is an unvalidated `String`, not an enum) is
/// spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-unit payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-unit payload reparses");
    assert_eq!(reencoded, original, "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the committed change-unit JSON is not canonical");
}

/// 🎯️ `unit` is a free-form `String`, so `change-unit` carries only the equality guard;
/// "calcium_silicate" differs from the committed "clay", so it stays shut.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: `change-unit` has no numeric-finiteness guard at all — only the equality guard — and "calcium_silicate" differs from the committed committed "clay", so `mutation.no-op` must not fire");
    assert!(produced.messages().is_empty(), "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: an accepted change-unit emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-unit` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `unit` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-unit diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the unit material and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-unit diff decodes");
    assert_eq!(decoded.unit, Some("calcium_silicate".to_string()), "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the committed diff must carry unit = "calcium_silicate"");
    assert!(decoded.mortar.is_none(), "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: change-unit writes unit and must leave `mortar` untouched");
    assert!(decoded.exposure.is_none(), "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: change-unit writes unit and must leave `exposure` untouched");
    assert!(decoded.artifact.is_none(), "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the unit-material switch, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-unit diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: the committed diff did not carry before to after");
    assert_eq!(produced.unit, "calcium_silicate", "change-unit/switches-the-masonry-unit-from-clay-to-calcium-silicate: applying the committed diff must land unit on "calcium_silicate"");
}
