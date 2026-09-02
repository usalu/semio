//! 🧪️ `change-t-ef-mm` fixture — `raises-the-effective-thickness-to-300-mm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-t-ef-mm` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1996Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1996Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1996Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-t-ef-mm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising t_ef from 240.0 mm to 300.0 mm rewrites `t_ef_mm` alone. Φ_s climbs from 0.6588 to 0.7736 as the
/// slenderness ratio falls, but the effective height feeding that ratio is untouched — and so is the
/// separately declared BUILT wall thickness.
#[semio_framework_async_macros::async_test]
fn raises_the_effective_thickness_to_300_mm() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-t-ef-mm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.t_ef_mm, 300.0, "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: t_ef_mm must read 300.0 mm once the change lands");
    assert_eq!(
        applied.wall_thickness_mm,
        before().wall_thickness_mm,
        "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the EN 1996-1-2 built thickness and the EN 1996-3 effective thickness are two independent document fields that happen to start equal"
    );
}

/// ↩️ `change-t-ef-mm`'s inverse reads the OLD 240.0 mm out of BASE, so replaying it puts the 240.0 mm effective
/// thickness back on `t_ef_mm`.
#[semio_framework_async_macros::async_test]
fn restoring_240_mm_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-t-ef-mm applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the inverse of one change-t-ef-mm is exactly one change-t-ef-mm back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-t-ef-mm inverse step applies");
    }
    assert_eq!(snapshot.t_ef_mm, base.t_ef_mm, "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the inverse must put the 240.0 mm effective thickness back on `t_ef_mm`");
    assert_eq!(snapshot, base, "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-t-ef-mm` payload are already canonical: decode → encode
/// is a fixed point, so `newTEfMm` (serde camelCase over `new_t_ef_mm`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-t-ef-mm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-t-ef-mm payload reparses");
    assert_eq!(reencoded, original, "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the committed change-t-ef-mm JSON is not canonical");
}

/// 🎯️ 300.0 mm is finite and differs from the committed 240.0 mm, so `change-t-ef-mm` returns a
/// clean outcome with an empty message list.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 300.0 differs from the committed 240.0, so the `mutation.no-op` warning guard stays shut too"
    );
    assert!(produced.messages().is_empty(), "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: an accepted change-t-ef-mm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-t-ef-mm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `tEfMm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-t-ef-mm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the effective thickness and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-t-ef-mm diff decodes");
    assert_eq!(decoded.t_ef_mm, Some(300.0), "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the committed diff must carry tEfMm = 300.0 mm");
    assert!(decoded.wall_thickness_mm.is_none(), "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: change-t-ef-mm writes tEfMm and must leave `wall_thickness_mm` untouched");
    assert!(decoded.h_ef_mm.is_none(), "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: change-t-ef-mm writes tEfMm and must leave `h_ef_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the effective-thickness change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-t-ef-mm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: the committed diff did not carry before to after");
    assert_eq!(produced.t_ef_mm, 300.0, "change-t-ef-mm/raises-the-effective-thickness-to-300-mm: applying the committed diff must land t_ef_mm on 300.0 mm");
}
