//! 🧪️ `change-wall-phi-deg` fixture — `📐️raises-wall-phi-deg-to-37-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.wall_phi_deg` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.wall_phi_deg == payload.new_wall_phi_deg` `mutation.no-op` guard.
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1998Snapshot {
    serde_json::from_str(BEFORE).expect("change-wall-phi-deg before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-wall-phi-deg after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-wall-phi-deg mutation decodes")
}

/// ▶️ `change-wall-phi-deg` carries the committed before-snapshot to the committed after-snapshot by moving
/// `wall_phi_deg` from 30.0 to 37.5, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_wall_phi_deg_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-wall-phi-deg applies to its committed before-snapshot");
    assert_eq!(applied.wall_phi_deg, 37.5, "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: wall_phi_deg must read 37.5 after the change");
    assert_eq!(applied, expected_after(), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: a real 30.0 to 37.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-wall-phi-deg` is its own inverse partner: the inverse step restores `wall_phi_deg` to its pre-change
/// 30.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_wall_phi_deg_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-wall-phi-deg applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-wall-phi-deg step applies");
        restored = next;
    }
    assert_eq!(restored.wall_phi_deg, 30.0, "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: the inverse must put wall_phi_deg back to 30.0");
    assert_eq!(restored, base, "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeWallPhiDeg` payload are already canonical:
/// decode then encode is a fixed point, so `wallPhiDeg` and `newWallPhiDeg` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_wall_phi_deg_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-wall-phi-deg snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-wall-phi-deg snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-wall-phi-deg snapshot reparses");
        assert_eq!(reencoded, original, "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-wall-phi-deg mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-wall-phi-deg mutation reparses");
    assert_eq!(reencoded, original, "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: committed mutation JSON is not the canonical externally-tagged ChangeWallPhiDeg form carrying newWallPhiDeg");
}

/// 🎯️ The declared outcome holds: `change-wall-phi-deg` at 37.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_wall_phi_deg_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-wall-phi-deg outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: moving wall_phi_deg from 30.0 to 37.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-wall-phi-deg` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `wallPhiDeg` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `wallSoilGammaKnM3`.
#[semio_framework_async_macros::async_test]
fn change_wall_phi_deg_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().wall_phi_deg, Some(37.5), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: the diff must set wall_phi_deg to 37.5");
    assert!(outcome.diff().artifact.is_none(), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().wall_soil_gamma_kn_m3.is_none(), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: change-wall-phi-deg must leave wall_soil_gamma_kn_m3 untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-wall-phi-deg produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-wall-phi-deg committed diff decodes");
    assert_eq!(produced, committed, "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `wallPhiDeg` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-wall-phi-deg`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_wall_phi_deg_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-wall-phi-deg committed diff decodes");
    assert_eq!(decoded.wall_phi_deg, Some(37.5), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: the committed diff must carry wall_phi_deg at 37.5");
    assert!(decoded.selected_check_index.is_none(), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-wall-phi-deg committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-wall-phi-deg committed diff reparses");
    assert_eq!(reencoded, original, "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 30.0 to 37.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_wall_phi_deg_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-wall-phi-deg committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-wall-phi-deg committed diff applies to the before-snapshot");
    assert_eq!(produced.wall_phi_deg, 37.5, "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: the committed diff must leave wall_phi_deg reading 37.5");
    assert_eq!(produced, expected_after(), "change-wall-phi-deg/raises-wall-phi-deg-to-37-5: the committed diff did not carry before to after");
}
