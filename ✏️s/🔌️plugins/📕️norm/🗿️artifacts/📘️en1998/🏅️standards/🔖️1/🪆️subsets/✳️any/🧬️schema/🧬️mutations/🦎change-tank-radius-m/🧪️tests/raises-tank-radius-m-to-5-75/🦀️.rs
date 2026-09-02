//! 🧪️ `change-tank-radius-m` fixture — `raises-tank-radius-m-to-5-75`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.tank_radius_m` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.tank_radius_m == payload.new_tank_radius_m` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-tank-radius-m before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-tank-radius-m after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-tank-radius-m mutation decodes")
}

/// ▶️ `change-tank-radius-m` carries the committed before-snapshot to the committed after-snapshot by moving
/// `tank_radius_m` from 4.0 to 5.75, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_tank_radius_m_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-tank-radius-m applies to its committed before-snapshot");
    assert_eq!(applied.tank_radius_m, 5.75, "change-tank-radius-m/raises-tank-radius-m-to-5-75: tank_radius_m must read 5.75 after the change");
    assert_eq!(applied, expected_after(), "change-tank-radius-m/raises-tank-radius-m-to-5-75: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-tank-radius-m/raises-tank-radius-m-to-5-75: a real 4.0 to 5.75 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-tank-radius-m` is its own inverse partner: the inverse step restores `tank_radius_m` to its pre-change
/// 4.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_tank_radius_m_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-tank-radius-m applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-tank-radius-m/raises-tank-radius-m-to-5-75: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-tank-radius-m step applies");
        restored = next;
    }
    assert_eq!(restored.tank_radius_m, 4.0, "change-tank-radius-m/raises-tank-radius-m-to-5-75: the inverse must put tank_radius_m back to 4.0");
    assert_eq!(restored, base, "change-tank-radius-m/raises-tank-radius-m-to-5-75: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeTankRadiusM` payload are already canonical:
/// decode then encode is a fixed point, so `tankRadiusM` and `newTankRadiusM` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_tank_radius_m_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-tank-radius-m snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-tank-radius-m snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-tank-radius-m snapshot reparses");
        assert_eq!(reencoded, original, "change-tank-radius-m/raises-tank-radius-m-to-5-75: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-tank-radius-m mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-tank-radius-m mutation reparses");
    assert_eq!(reencoded, original, "change-tank-radius-m/raises-tank-radius-m-to-5-75: committed mutation JSON is not the canonical externally-tagged ChangeTankRadiusM form carrying newTankRadiusM");
}

/// 🎯️ The declared outcome holds: `change-tank-radius-m` at 5.75 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_tank_radius_m_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-tank-radius-m outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-tank-radius-m/raises-tank-radius-m-to-5-75: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-tank-radius-m/raises-tank-radius-m-to-5-75: moving tank_radius_m from 4.0 to 5.75 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-tank-radius-m/raises-tank-radius-m-to-5-75: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-tank-radius-m` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `tankRadiusM` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `tankMassT`.
#[semio_framework_async_macros::async_test]
fn change_tank_radius_m_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().tank_radius_m, Some(5.75), "change-tank-radius-m/raises-tank-radius-m-to-5-75: the diff must set tank_radius_m to 5.75");
    assert!(outcome.diff().artifact.is_none(), "change-tank-radius-m/raises-tank-radius-m-to-5-75: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().tank_mass_t.is_none(), "change-tank-radius-m/raises-tank-radius-m-to-5-75: change-tank-radius-m must leave tank_mass_t untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-tank-radius-m produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-tank-radius-m committed diff decodes");
    assert_eq!(produced, committed, "change-tank-radius-m/raises-tank-radius-m-to-5-75: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `tankRadiusM` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-tank-radius-m`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_tank_radius_m_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-tank-radius-m committed diff decodes");
    assert_eq!(decoded.tank_radius_m, Some(5.75), "change-tank-radius-m/raises-tank-radius-m-to-5-75: the committed diff must carry tank_radius_m at 5.75");
    assert!(decoded.selected_check_index.is_none(), "change-tank-radius-m/raises-tank-radius-m-to-5-75: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-tank-radius-m committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-tank-radius-m committed diff reparses");
    assert_eq!(reencoded, original, "change-tank-radius-m/raises-tank-radius-m-to-5-75: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 4.0 to 5.75 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_tank_radius_m_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-tank-radius-m committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-tank-radius-m committed diff applies to the before-snapshot");
    assert_eq!(produced.tank_radius_m, 5.75, "change-tank-radius-m/raises-tank-radius-m-to-5-75: the committed diff must leave tank_radius_m reading 5.75");
    assert_eq!(produced, expected_after(), "change-tank-radius-m/raises-tank-radius-m-to-5-75: the committed diff did not carry before to after");
}
