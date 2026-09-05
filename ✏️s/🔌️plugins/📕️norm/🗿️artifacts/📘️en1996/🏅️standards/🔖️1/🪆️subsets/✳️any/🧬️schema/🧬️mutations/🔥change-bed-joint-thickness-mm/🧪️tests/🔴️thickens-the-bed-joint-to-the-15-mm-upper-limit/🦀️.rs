//! 🧪️ `change-bed-joint-thickness-mm` fixture — `🔴️thickens-the-bed-joint-to-the-15-mm-upper-limit`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-bed-joint-thickness-mm` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-bed-joint-thickness-mm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Thickening the bed joint from 12.0 mm to 15.0 mm — the top of the EN 1996-2 §8 6–15 mm general-purpose
/// band — rewrites `bed_joint_thickness_mm` alone.
#[semio_framework_async_macros::async_test]
fn thickens_the_bed_joint_to_the_15_mm_upper_limit() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-bed-joint-thickness-mm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.bed_joint_thickness_mm, 15.0, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: bed_joint_thickness_mm must read 15.0 mm once the change lands");
    assert_eq!(applied.mortar, before().mortar, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the mortar class is the OTHER half of the execution facet and must not follow a joint-geometry edit");
}

/// ↩️ `change-bed-joint-thickness-mm`'s inverse reads the OLD 12.0 mm out of BASE, so replaying it puts the 12.0
/// mm bed joint back on `bed_joint_thickness_mm`.
#[semio_framework_async_macros::async_test]
fn restoring_12_mm_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-bed-joint-thickness-mm applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the inverse of one change-bed-joint-thickness-mm is exactly one change-bed-joint-thickness-mm back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-bed-joint-thickness-mm inverse step applies");
    }
    assert_eq!(snapshot.bed_joint_thickness_mm, base.bed_joint_thickness_mm, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the inverse must put the 12.0 mm bed joint back on `bed_joint_thickness_mm`");
    assert_eq!(snapshot, base, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-bed-joint-thickness-mm` payload are already canonical:
/// decode → encode is a fixed point, so `newBedJointThicknessMm` (serde camelCase over
/// `new_bed_joint_thickness_mm`) is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-bed-joint-thickness-mm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-bed-joint-thickness-mm payload reparses");
    assert_eq!(reencoded, original, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the committed change-bed-joint-thickness-mm JSON is not canonical");
}

/// 🎯️ 15.0 mm is finite and differs from the committed 12.0 mm. The 6–15 mm band is enforced by
/// the EN 1996-2 §8 CHECK, not by this mutation — its diff builder has no range guard.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 15.0 differs from the committed 12.0, so the `mutation.no-op` warning guard stays shut too");
    assert!(produced.messages().is_empty(), "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: an accepted change-bed-joint-thickness-mm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-bed-joint-thickness-mm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `bedJointThicknessMm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-bed-joint-thickness-mm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the bed-joint thickness and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-bed-joint-thickness-mm diff decodes");
    assert_eq!(decoded.bed_joint_thickness_mm, Some(15.0), "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the committed diff must carry bedJointThicknessMm = 15.0 mm");
    assert!(decoded.mortar.is_none(), "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: change-bed-joint-thickness-mm writes bedJointThicknessMm and must leave `mortar` untouched");
    assert!(decoded.wall_thickness_mm.is_none(), "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: change-bed-joint-thickness-mm writes bedJointThicknessMm and must leave `wall_thickness_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the bed-joint-thickness change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-bed-joint-thickness-mm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: the committed diff did not carry before to after");
    assert_eq!(produced.bed_joint_thickness_mm, 15.0, "change-bed-joint-thickness-mm/thickens-the-bed-joint-to-the-15-mm-upper-limit: applying the committed diff must land bed_joint_thickness_mm on 15.0 mm");
}
