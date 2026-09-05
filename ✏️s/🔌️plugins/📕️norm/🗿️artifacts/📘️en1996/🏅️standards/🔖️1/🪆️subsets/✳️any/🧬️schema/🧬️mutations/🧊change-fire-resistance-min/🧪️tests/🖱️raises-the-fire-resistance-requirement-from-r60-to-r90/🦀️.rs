//! 🧪️ `change-fire-resistance-min` fixture — `🖱️raises-the-fire-resistance-requirement-from-r60-to-r90`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-fire-resistance-min` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-fire-resistance-min` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising the required fire resistance from R60 to R90 rewrites `fire_resistance_min` alone. EN 1996-1-2
/// Table 5.1 then demands 120 mm rather than 90 mm of clay masonry, but that REQUIRED thickness is computed,
/// never stored, so the built thickness is untouched.
#[semio_framework_async_macros::async_test]
fn raises_the_fire_resistance_requirement_from_r60_to_r90() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-fire-resistance-min applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.fire_resistance_min, 90, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: fire_resistance_min must read 90 minutes once the change lands");
    assert_eq!(
        applied.wall_thickness_mm,
        before().wall_thickness_mm,
        "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the built wall thickness is what the fire check COMPARES against and must never be auto-raised to satisfy the new requirement"
    );
}

/// ↩️ `change-fire-resistance-min`'s inverse reads the OLD 60 minutes out of BASE, so replaying it puts the R60
/// requirement back on `fire_resistance_min`.
#[semio_framework_async_macros::async_test]
fn returning_to_r60_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-fire-resistance-min applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the inverse of one change-fire-resistance-min is exactly one change-fire-resistance-min back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-fire-resistance-min inverse step applies");
    }
    assert_eq!(snapshot.fire_resistance_min, base.fire_resistance_min, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the inverse must put the R60 requirement back on `fire_resistance_min`");
    assert_eq!(snapshot, base, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-fire-resistance-min` payload are already canonical:
/// decode → encode is a fixed point, so `newFireResistanceMin`, carrying a bare JSON integer because the
/// field is a `u32` is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-fire-resistance-min payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-fire-resistance-min payload reparses");
    assert_eq!(reencoded, original, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the committed change-fire-resistance-min JSON is not canonical");
}

/// 🎯️ `fire_resistance_min` is a `u32`, so `change-fire-resistance-min` has no finiteness guard;
/// 90 differs from the committed 60, so its only guard — the equality one — stays shut.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: `change-fire-resistance-min` has no numeric-finiteness guard at all — only the equality guard — and 90 differs from the committed committed 60, so `mutation.no-op` must not fire");
    assert!(produced.messages().is_empty(), "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: an accepted change-fire-resistance-min emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-fire-resistance-min` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `fireResistanceMin` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-fire-resistance-min diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the fire-resistance
/// requirement and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-fire-resistance-min diff decodes");
    assert_eq!(decoded.fire_resistance_min, Some(90), "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the committed diff must carry fireResistanceMin = 90 minutes");
    assert!(decoded.wall_thickness_mm.is_none(), "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: change-fire-resistance-min writes fireResistanceMin and must leave `wall_thickness_mm` untouched");
    assert!(decoded.unit.is_none(), "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: change-fire-resistance-min writes fireResistanceMin and must leave `unit` untouched");
    assert!(decoded.artifact.is_none(), "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the fire-requirement change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-fire-resistance-min diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: the committed diff did not carry before to after");
    assert_eq!(produced.fire_resistance_min, 90, "change-fire-resistance-min/raises-the-fire-resistance-requirement-from-r60-to-r90: applying the committed diff must land fire_resistance_min on 90 minutes");
}
