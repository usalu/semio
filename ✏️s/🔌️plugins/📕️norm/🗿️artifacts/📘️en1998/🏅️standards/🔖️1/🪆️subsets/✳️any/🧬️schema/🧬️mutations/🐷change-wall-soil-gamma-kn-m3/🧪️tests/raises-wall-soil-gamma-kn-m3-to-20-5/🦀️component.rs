//! 🧪️ `change-wall-soil-gamma-kn-m3` fixture — `raises-wall-soil-gamma-kn-m3-to-20-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.wall_soil_gamma_kn_m3` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.wall_soil_gamma_kn_m3 == payload.new_wall_soil_gamma_kn_m3` `mutation.no-op` guard.
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1998Snapshot {
    serde_json::from_str(BEFORE).expect("change-wall-soil-gamma-kn-m3 before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-wall-soil-gamma-kn-m3 after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-wall-soil-gamma-kn-m3 mutation decodes")
}

/// ▶️ `change-wall-soil-gamma-kn-m3` carries the committed before-snapshot to the committed after-snapshot by moving
/// `wall_soil_gamma_kn_m3` from 18.0 to 20.5, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_wall_soil_gamma_kn_m3_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-wall-soil-gamma-kn-m3 applies to its committed before-snapshot");
    assert_eq!(applied.wall_soil_gamma_kn_m3, 20.5, "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: wall_soil_gamma_kn_m3 must read 20.5 after the change");
    assert_eq!(applied, expected_after(), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: a real 18.0 to 20.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-wall-soil-gamma-kn-m3` is its own inverse partner: the inverse step restores `wall_soil_gamma_kn_m3` to its pre-change
/// 18.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_wall_soil_gamma_kn_m3_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-wall-soil-gamma-kn-m3 applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-wall-soil-gamma-kn-m3 step applies");
        restored = next;
    }
    assert_eq!(restored.wall_soil_gamma_kn_m3, 18.0, "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: the inverse must put wall_soil_gamma_kn_m3 back to 18.0");
    assert_eq!(restored, base, "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeWallSoilGammaKnM3` payload are already canonical:
/// decode then encode is a fixed point, so `wallSoilGammaKnM3` and `newWallSoilGammaKnM3` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_wall_soil_gamma_kn_m3_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-wall-soil-gamma-kn-m3 snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-wall-soil-gamma-kn-m3 snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-wall-soil-gamma-kn-m3 snapshot reparses");
        assert_eq!(reencoded, original, "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-wall-soil-gamma-kn-m3 mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-wall-soil-gamma-kn-m3 mutation reparses");
    assert_eq!(reencoded, original, "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: committed mutation JSON is not the canonical externally-tagged ChangeWallSoilGammaKnM3 form carrying newWallSoilGammaKnM3");
}

/// 🎯️ The declared outcome holds: `change-wall-soil-gamma-kn-m3` at 20.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_wall_soil_gamma_kn_m3_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-wall-soil-gamma-kn-m3 outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: moving wall_soil_gamma_kn_m3 from 18.0 to 20.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-wall-soil-gamma-kn-m3` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `wallSoilGammaKnM3` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `wallR`.
#[semio_framework_async_macros::async_test]
async fn change_wall_soil_gamma_kn_m3_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().wall_soil_gamma_kn_m3, Some(20.5), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: the diff must set wall_soil_gamma_kn_m3 to 20.5");
    assert!(outcome.diff().artifact.is_none(), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().wall_r.is_none(), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: change-wall-soil-gamma-kn-m3 must leave wall_r untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-wall-soil-gamma-kn-m3 produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-wall-soil-gamma-kn-m3 committed diff decodes");
    assert_eq!(produced, committed, "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `wallSoilGammaKnM3` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-wall-soil-gamma-kn-m3`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_wall_soil_gamma_kn_m3_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-wall-soil-gamma-kn-m3 committed diff decodes");
    assert_eq!(decoded.wall_soil_gamma_kn_m3, Some(20.5), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: the committed diff must carry wall_soil_gamma_kn_m3 at 20.5");
    assert!(decoded.selected_check_index.is_none(), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-wall-soil-gamma-kn-m3 committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-wall-soil-gamma-kn-m3 committed diff reparses");
    assert_eq!(reencoded, original, "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 18.0 to 20.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_wall_soil_gamma_kn_m3_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-wall-soil-gamma-kn-m3 committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-wall-soil-gamma-kn-m3 committed diff applies to the before-snapshot");
    assert_eq!(produced.wall_soil_gamma_kn_m3, 20.5, "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: the committed diff must leave wall_soil_gamma_kn_m3 reading 20.5");
    assert_eq!(produced, expected_after(), "change-wall-soil-gamma-kn-m3/raises-wall-soil-gamma-kn-m3-to-20-5: the committed diff did not carry before to after");
}
