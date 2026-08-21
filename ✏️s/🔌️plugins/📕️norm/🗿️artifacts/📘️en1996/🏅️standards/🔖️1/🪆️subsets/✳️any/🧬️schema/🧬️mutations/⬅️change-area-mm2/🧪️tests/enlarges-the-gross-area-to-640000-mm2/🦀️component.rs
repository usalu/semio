//! 🧪️ `change-area-mm2` fixture — `enlarges-the-gross-area-to-640000-mm2`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-area-mm2` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-area-mm2` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Enlarging the gross area from 500000.0 mm² to 640000.0 mm² rewrites `area_mm2` alone — the shear area is a
/// separate, independently entered field and does not track it.
#[semio_framework_async_macros::async_test]
async fn enlarges_the_gross_area_to_640000_mm2() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-area-mm2 applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.area_mm2, 640000.0, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: area_mm2 must read 640000.0 mm² once the change lands");
    assert_eq!(applied.shear_area_mm2, before().shear_area_mm2, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the shear area is its own document field and must not be recomputed from the gross area");
}

/// ↩️ `change-area-mm2`'s inverse reads the OLD 500000.0 mm² out of BASE, so replaying it puts the 500000.0 mm²
/// back on `area_mm2`.
#[semio_framework_async_macros::async_test]
async fn restoring_500000_mm2_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-area-mm2 applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the inverse of one change-area-mm2 is exactly one change-area-mm2 back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-area-mm2 inverse step applies");
    }
    assert_eq!(snapshot.area_mm2, base.area_mm2, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the inverse must put the 500000.0 mm² back on `area_mm2`");
    assert_eq!(snapshot, base, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-area-mm2` payload are already canonical: decode →
/// encode is a fixed point, so `newAreaMm2` (serde camelCase over `new_area_mm2`) is spelled here exactly as
/// this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-area-mm2 payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-area-mm2 payload reparses");
    assert_eq!(reencoded, original, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the committed change-area-mm2 JSON is not canonical");
}

/// 🎯️ 640000.0 mm² is finite and differs from the committed 500000.0 mm², so `change-area-mm2`
/// returns a clean, message-free outcome.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 640000.0 differs from the committed 500000.0, so the `mutation.no-op` warning guard stays shut too");
    assert!(produced.messages().is_empty(), "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: an accepted change-area-mm2 emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-area-mm2` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `areaMm2` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-area-mm2 diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the gross cross-sectional
/// area and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-area-mm2 diff decodes");
    assert_eq!(decoded.area_mm2, Some(640000.0), "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the committed diff must carry areaMm2 = 640000.0 mm²");
    assert!(decoded.shear_area_mm2.is_none(), "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: change-area-mm2 writes areaMm2 and must leave `shear_area_mm2` untouched");
    assert!(decoded.wall_thickness_mm.is_none(), "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: change-area-mm2 writes areaMm2 and must leave `wall_thickness_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the gross-area change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-area-mm2 diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: the committed diff did not carry before to after");
    assert_eq!(produced.area_mm2, 640000.0, "change-area-mm2/enlarges-the-gross-area-to-640000-mm2: applying the committed diff must land area_mm2 on 640000.0 mm²");
}
