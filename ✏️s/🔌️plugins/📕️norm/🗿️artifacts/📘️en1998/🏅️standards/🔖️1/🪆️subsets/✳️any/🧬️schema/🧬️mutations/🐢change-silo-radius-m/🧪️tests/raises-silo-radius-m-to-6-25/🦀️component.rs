//! 🧪️ `change-silo-radius-m` fixture — `raises-silo-radius-m-to-6-25`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.silo_radius_m` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.silo_radius_m == payload.new_silo_radius_m` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-silo-radius-m before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-silo-radius-m after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-silo-radius-m mutation decodes")
}

/// ▶️ `change-silo-radius-m` carries the committed before-snapshot to the committed after-snapshot by moving
/// `silo_radius_m` from 5.0 to 6.25, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_silo_radius_m_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-silo-radius-m applies to its committed before-snapshot");
    assert_eq!(applied.silo_radius_m, 6.25, "change-silo-radius-m/raises-silo-radius-m-to-6-25: silo_radius_m must read 6.25 after the change");
    assert_eq!(applied, expected_after(), "change-silo-radius-m/raises-silo-radius-m-to-6-25: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-silo-radius-m/raises-silo-radius-m-to-6-25: a real 5.0 to 6.25 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-silo-radius-m` is its own inverse partner: the inverse step restores `silo_radius_m` to its pre-change
/// 5.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_silo_radius_m_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-silo-radius-m applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-silo-radius-m/raises-silo-radius-m-to-6-25: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-silo-radius-m step applies");
        restored = next;
    }
    assert_eq!(restored.silo_radius_m, 5.0, "change-silo-radius-m/raises-silo-radius-m-to-6-25: the inverse must put silo_radius_m back to 5.0");
    assert_eq!(restored, base, "change-silo-radius-m/raises-silo-radius-m-to-6-25: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeSiloRadiusM` payload are already canonical:
/// decode then encode is a fixed point, so `siloRadiusM` and `newSiloRadiusM` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_silo_radius_m_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-silo-radius-m snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-silo-radius-m snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-silo-radius-m snapshot reparses");
        assert_eq!(reencoded, original, "change-silo-radius-m/raises-silo-radius-m-to-6-25: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-silo-radius-m mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-silo-radius-m mutation reparses");
    assert_eq!(reencoded, original, "change-silo-radius-m/raises-silo-radius-m-to-6-25: committed mutation JSON is not the canonical externally-tagged ChangeSiloRadiusM form carrying newSiloRadiusM");
}

/// 🎯️ The declared outcome holds: `change-silo-radius-m` at 6.25 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_silo_radius_m_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-silo-radius-m outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-silo-radius-m/raises-silo-radius-m-to-6-25: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-silo-radius-m/raises-silo-radius-m-to-6-25: moving silo_radius_m from 5.0 to 6.25 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-silo-radius-m/raises-silo-radius-m-to-6-25: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-silo-radius-m` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `siloRadiusM` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `siloNRdKn`.
#[semio_framework_async_macros::async_test]
fn change_silo_radius_m_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().silo_radius_m, Some(6.25), "change-silo-radius-m/raises-silo-radius-m-to-6-25: the diff must set silo_radius_m to 6.25");
    assert!(outcome.diff().artifact.is_none(), "change-silo-radius-m/raises-silo-radius-m-to-6-25: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().silo_n_rd_kn.is_none(), "change-silo-radius-m/raises-silo-radius-m-to-6-25: change-silo-radius-m must leave silo_n_rd_kn untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-silo-radius-m produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-radius-m committed diff decodes");
    assert_eq!(produced, committed, "change-silo-radius-m/raises-silo-radius-m-to-6-25: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `siloRadiusM` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-silo-radius-m`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_silo_radius_m_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-silo-radius-m committed diff decodes");
    assert_eq!(decoded.silo_radius_m, Some(6.25), "change-silo-radius-m/raises-silo-radius-m-to-6-25: the committed diff must carry silo_radius_m at 6.25");
    assert!(decoded.selected_check_index.is_none(), "change-silo-radius-m/raises-silo-radius-m-to-6-25: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-silo-radius-m committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-radius-m committed diff reparses");
    assert_eq!(reencoded, original, "change-silo-radius-m/raises-silo-radius-m-to-6-25: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 5.0 to 6.25 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_silo_radius_m_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-silo-radius-m committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-silo-radius-m committed diff applies to the before-snapshot");
    assert_eq!(produced.silo_radius_m, 6.25, "change-silo-radius-m/raises-silo-radius-m-to-6-25: the committed diff must leave silo_radius_m reading 6.25");
    assert_eq!(produced, expected_after(), "change-silo-radius-m/raises-silo-radius-m-to-6-25: the committed diff did not carry before to after");
}
