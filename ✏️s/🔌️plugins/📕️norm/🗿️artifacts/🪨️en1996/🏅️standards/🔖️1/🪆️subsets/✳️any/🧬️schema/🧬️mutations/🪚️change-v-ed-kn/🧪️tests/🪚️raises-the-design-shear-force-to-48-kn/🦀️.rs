//! 🧪️ `change-v-ed-kn` fixture — `🪚️raises-the-design-shear-force-to-48-kn`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-v-ed-kn` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-v-ed-kn` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising V_Ed from 35.0 kN to 48.0 kN rewrites `v_ed_kn` alone — the shear area that the §6.2.3 resistance
/// is built from stays as committed.
#[semio_framework_async_macros::async_test]
fn raises_the_design_shear_force_to_48_kn() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-v-ed-kn applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.v_ed_kn, 48.0, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: v_ed_kn must read 48.0 kN once the change lands");
    assert_eq!(applied.shear_area_mm2, before().shear_area_mm2, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the shear area belongs to the resistance side of the same check and must not move with the action");
}

/// ↩️ `change-v-ed-kn`'s inverse reads the OLD 35.0 kN out of BASE, so replaying it puts the 35.0 kN back on
/// `v_ed_kn`.
#[semio_framework_async_macros::async_test]
fn restoring_35_kn_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-v-ed-kn applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the inverse of one change-v-ed-kn is exactly one change-v-ed-kn back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-v-ed-kn inverse step applies");
    }
    assert_eq!(snapshot.v_ed_kn, base.v_ed_kn, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the inverse must put the 35.0 kN back on `v_ed_kn`");
    assert_eq!(snapshot, base, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-v-ed-kn` payload are already canonical: decode → encode
/// is a fixed point, so `newVEdKn` (serde camelCase over `new_v_ed_kn`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-v-ed-kn payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-v-ed-kn payload reparses");
    assert_eq!(reencoded, original, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the committed change-v-ed-kn JSON is not canonical");
}

/// 🎯️ 48.0 kN is finite and differs from the committed 35.0 kN, so neither guard in
/// `change-v-ed-kn`'s diff builder fires.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 48.0 differs from the committed 35.0, so the `mutation.no-op` warning guard stays shut too"
    );
    assert!(produced.messages().is_empty(), "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: an accepted change-v-ed-kn emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-v-ed-kn` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `vEdKn` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-v-ed-kn diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the design shear force and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-v-ed-kn diff decodes");
    assert_eq!(decoded.v_ed_kn, Some(48.0), "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the committed diff must carry vEdKn = 48.0 kN");
    assert!(decoded.shear_area_mm2.is_none(), "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: change-v-ed-kn writes vEdKn and must leave `shear_area_mm2` untouched");
    assert!(decoded.f_vk_mpa.is_none(), "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: change-v-ed-kn writes vEdKn and must leave `f_vk_mpa` untouched");
    assert!(decoded.artifact.is_none(), "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the shear-force change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-v-ed-kn diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: the committed diff did not carry before to after");
    assert_eq!(produced.v_ed_kn, 48.0, "change-v-ed-kn/raises-the-design-shear-force-to-48-kn: applying the committed diff must land v_ed_kn on 48.0 kN");
}
