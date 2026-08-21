//! 🧪️ `change-h-ef-mm` fixture — `lengthens-the-effective-height-to-2750-mm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-h-ef-mm` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-h-ef-mm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Lengthening h_ef from 2500.0 mm to 2750.0 mm rewrites `h_ef_mm` alone. The slenderness ratio h_ef/t_ef
/// climbs from 10.42 to 11.46 — still well inside the EN 1996-3 limit of 27 — but t_ef is the denominator and
/// stays as committed.
#[semio_framework_async_macros::async_test]
async fn lengthens_the_effective_height_to_2750_mm() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-h-ef-mm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.h_ef_mm, 2750.0, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: h_ef_mm must read 2750.0 mm once the change lands");
    assert_eq!(applied.t_ef_mm, before().t_ef_mm, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the effective thickness is the denominator of the very slenderness ratio this edit moves and must not be rescaled with it");
}

/// ↩️ `change-h-ef-mm`'s inverse reads the OLD 2500.0 mm out of BASE, so replaying it puts the 2500.0 mm
/// effective height back on `h_ef_mm`.
#[semio_framework_async_macros::async_test]
async fn restoring_2500_mm_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-h-ef-mm applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the inverse of one change-h-ef-mm is exactly one change-h-ef-mm back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-h-ef-mm inverse step applies");
    }
    assert_eq!(snapshot.h_ef_mm, base.h_ef_mm, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the inverse must put the 2500.0 mm effective height back on `h_ef_mm`");
    assert_eq!(snapshot, base, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-h-ef-mm` payload are already canonical: decode → encode
/// is a fixed point, so `newHEfMm` (serde camelCase over `new_h_ef_mm`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-h-ef-mm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-h-ef-mm payload reparses");
    assert_eq!(reencoded, original, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the committed change-h-ef-mm JSON is not canonical");
}

/// 🎯️ 2750.0 mm is finite and differs from the committed 2500.0 mm, so `change-h-ef-mm` emits
/// no message. The slenderness ceiling of 27 is enforced by `part_3::is_applicable`, not here.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 2750.0 differs from the committed 2500.0, so the `mutation.no-op` warning guard stays shut too");
    assert!(produced.messages().is_empty(), "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: an accepted change-h-ef-mm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-h-ef-mm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `hEfMm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-h-ef-mm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the effective height and
/// nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-h-ef-mm diff decodes");
    assert_eq!(decoded.h_ef_mm, Some(2750.0), "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the committed diff must carry hEfMm = 2750.0 mm");
    assert!(decoded.t_ef_mm.is_none(), "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: change-h-ef-mm writes hEfMm and must leave `t_ef_mm` untouched");
    assert!(decoded.storeys.is_none(), "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: change-h-ef-mm writes hEfMm and must leave `storeys` untouched");
    assert!(decoded.artifact.is_none(), "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the effective-height change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-h-ef-mm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: the committed diff did not carry before to after");
    assert_eq!(produced.h_ef_mm, 2750.0, "change-h-ef-mm/lengthens-the-effective-height-to-2750-mm: applying the committed diff must land h_ef_mm on 2750.0 mm");
}
