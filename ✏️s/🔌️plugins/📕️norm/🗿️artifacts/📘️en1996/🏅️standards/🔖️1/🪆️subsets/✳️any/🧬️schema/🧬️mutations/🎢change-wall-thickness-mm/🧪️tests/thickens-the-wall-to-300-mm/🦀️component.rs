//! 🧪️ `change-wall-thickness-mm` fixture — `thickens-the-wall-to-300-mm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-wall-thickness-mm` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-wall-thickness-mm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Thickening the wall from 240.0 mm to 300.0 mm rewrites `wall_thickness_mm` alone — the EFFECTIVE thickness
/// `t_ef_mm` that EN 1996-3 §4.2 slenderness uses is its own field and does not follow.
#[semio_framework_async_macros::async_test]
async fn thickens_the_wall_to_300_mm() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-wall-thickness-mm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.wall_thickness_mm, 300.0, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: wall_thickness_mm must read 300.0 mm once the change lands");
    assert_eq!(applied.t_ef_mm, before().t_ef_mm, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the effective thickness is a separately entered EN 1996-3 quantity and must not be re-derived from the built thickness");
}

/// ↩️ `change-wall-thickness-mm`'s inverse reads the OLD 240.0 mm out of BASE, so replaying it puts the 240.0 mm
/// back on `wall_thickness_mm`.
#[semio_framework_async_macros::async_test]
async fn restoring_240_mm_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-wall-thickness-mm applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the inverse of one change-wall-thickness-mm is exactly one change-wall-thickness-mm back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-wall-thickness-mm inverse step applies");
    }
    assert_eq!(snapshot.wall_thickness_mm, base.wall_thickness_mm, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the inverse must put the 240.0 mm back on `wall_thickness_mm`");
    assert_eq!(snapshot, base, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-wall-thickness-mm` payload are already canonical:
/// decode → encode is a fixed point, so `newWallThicknessMm` (serde camelCase over `new_wall_thickness_mm`)
/// is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-wall-thickness-mm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-wall-thickness-mm payload reparses");
    assert_eq!(reencoded, original, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the committed change-wall-thickness-mm JSON is not canonical");
}

/// 🎯️ 300.0 mm is finite and differs from the committed 240.0 mm, so the fire-check thickness
/// edit lands cleanly with no diagnostics.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-wall-thickness-mm/thickens-the-wall-to-300-mm: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 300.0 differs from the committed 240.0, so the `mutation.no-op` warning guard stays shut too");
    assert!(produced.messages().is_empty(), "change-wall-thickness-mm/thickens-the-wall-to-300-mm: an accepted change-wall-thickness-mm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-wall-thickness-mm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `wallThicknessMm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-wall-thickness-mm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the built wall thickness and
/// nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-wall-thickness-mm diff decodes");
    assert_eq!(decoded.wall_thickness_mm, Some(300.0), "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the committed diff must carry wallThicknessMm = 300.0 mm");
    assert!(decoded.t_ef_mm.is_none(), "change-wall-thickness-mm/thickens-the-wall-to-300-mm: change-wall-thickness-mm writes wallThicknessMm and must leave `t_ef_mm` untouched");
    assert!(decoded.fire_resistance_min.is_none(), "change-wall-thickness-mm/thickens-the-wall-to-300-mm: change-wall-thickness-mm writes wallThicknessMm and must leave `fire_resistance_min` untouched");
    assert!(decoded.artifact.is_none(), "change-wall-thickness-mm/thickens-the-wall-to-300-mm: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the wall-thickness change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-wall-thickness-mm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-wall-thickness-mm/thickens-the-wall-to-300-mm: the committed diff did not carry before to after");
    assert_eq!(produced.wall_thickness_mm, 300.0, "change-wall-thickness-mm/thickens-the-wall-to-300-mm: applying the committed diff must land wall_thickness_mm on 300.0 mm");
}
