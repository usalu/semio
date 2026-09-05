//! 🧪️ `change-mortar` fixture — `🪣️upgrades-the-general-purpose-mortar-to-m10`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-mortar` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-mortar` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Upgrading the mortar from M5 to M10 rewrites `mortar` alone — the exposure class that decides how much
/// mortar strength is required is untouched.
#[semio_framework_async_macros::async_test]
fn upgrades_the_general_purpose_mortar_to_m10() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-mortar applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.mortar, crate::artifacts::en1996::part_2::MortarClass::M10, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: mortar must read `MortarClass::M10` once the change lands");
    assert_eq!(applied.exposure, before().exposure, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the exposure class is the demand side of the EN 1996-2 Annex B admissibility check and must not move with the supply side");
}

/// ↩️ `change-mortar`'s inverse reads the OLD `MortarClass::M5` out of BASE, so replaying it puts the M5 mortar
/// back on `mortar`.
#[semio_framework_async_macros::async_test]
fn returning_to_m5_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-mortar applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the inverse of one change-mortar is exactly one change-mortar back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-mortar inverse step applies");
    }
    assert_eq!(snapshot.mortar, base.mortar, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the inverse must put the M5 mortar back on `mortar`");
    assert_eq!(snapshot, base, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-mortar` payload are already canonical: decode → encode
/// is a fixed point, so `"M10"` — `MortarClass` renames only `M2_5` (and only for the DSL key), never for
/// serde is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-mortar payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-mortar payload reparses");
    assert_eq!(reencoded, original, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the committed change-mortar JSON is not canonical");
}

/// 🎯️ `M10` differs from the committed `M5`, so the equality guard — `change-mortar`'s only
/// guard — stays shut.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-mortar/upgrades-the-general-purpose-mortar-to-m10: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: `change-mortar` has no numeric-finiteness guard at all — only the equality guard — and `MortarClass::M10` differs from the committed committed `M5`, so `mutation.no-op` must not fire");
    assert!(produced.messages().is_empty(), "change-mortar/upgrades-the-general-purpose-mortar-to-m10: an accepted change-mortar emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-mortar` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `mortar` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-mortar diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the mortar class and nothing
/// else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-mortar diff decodes");
    assert_eq!(decoded.mortar, Some(crate::artifacts::en1996::part_2::MortarClass::M10), "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the committed diff must carry mortar = `MortarClass::M10`");
    assert!(decoded.exposure.is_none(), "change-mortar/upgrades-the-general-purpose-mortar-to-m10: change-mortar writes mortar and must leave `exposure` untouched");
    assert!(decoded.bed_joint_thickness_mm.is_none(), "change-mortar/upgrades-the-general-purpose-mortar-to-m10: change-mortar writes mortar and must leave `bed_joint_thickness_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-mortar/upgrades-the-general-purpose-mortar-to-m10: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the mortar-class upgrade, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-mortar diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-mortar/upgrades-the-general-purpose-mortar-to-m10: the committed diff did not carry before to after");
    assert_eq!(produced.mortar, crate::artifacts::en1996::part_2::MortarClass::M10, "change-mortar/upgrades-the-general-purpose-mortar-to-m10: applying the committed diff must land mortar on `MortarClass::M10`");
}
