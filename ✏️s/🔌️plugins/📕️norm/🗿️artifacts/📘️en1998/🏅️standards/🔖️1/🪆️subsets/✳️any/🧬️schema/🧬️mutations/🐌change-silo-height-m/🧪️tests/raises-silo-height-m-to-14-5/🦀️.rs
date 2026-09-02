//! 🧪️ `change-silo-height-m` fixture — `raises-silo-height-m-to-14-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.silo_height_m` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.silo_height_m == payload.new_silo_height_m` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-silo-height-m before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-silo-height-m after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-silo-height-m mutation decodes")
}

/// ▶️ `change-silo-height-m` carries the committed before-snapshot to the committed after-snapshot by moving
/// `silo_height_m` from 10.0 to 14.5, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_silo_height_m_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-silo-height-m applies to its committed before-snapshot");
    assert_eq!(applied.silo_height_m, 14.5, "change-silo-height-m/raises-silo-height-m-to-14-5: silo_height_m must read 14.5 after the change");
    assert_eq!(applied, expected_after(), "change-silo-height-m/raises-silo-height-m-to-14-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-silo-height-m/raises-silo-height-m-to-14-5: a real 10.0 to 14.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-silo-height-m` is its own inverse partner: the inverse step restores `silo_height_m` to its pre-change
/// 10.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_silo_height_m_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-silo-height-m applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-silo-height-m/raises-silo-height-m-to-14-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-silo-height-m step applies");
        restored = next;
    }
    assert_eq!(restored.silo_height_m, 10.0, "change-silo-height-m/raises-silo-height-m-to-14-5: the inverse must put silo_height_m back to 10.0");
    assert_eq!(restored, base, "change-silo-height-m/raises-silo-height-m-to-14-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeSiloHeightM` payload are already canonical:
/// decode then encode is a fixed point, so `siloHeightM` and `newSiloHeightM` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_silo_height_m_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-silo-height-m snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-silo-height-m snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-silo-height-m snapshot reparses");
        assert_eq!(reencoded, original, "change-silo-height-m/raises-silo-height-m-to-14-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-silo-height-m mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-silo-height-m mutation reparses");
    assert_eq!(reencoded, original, "change-silo-height-m/raises-silo-height-m-to-14-5: committed mutation JSON is not the canonical externally-tagged ChangeSiloHeightM form carrying newSiloHeightM");
}

/// 🎯️ The declared outcome holds: `change-silo-height-m` at 14.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_silo_height_m_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-silo-height-m outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-silo-height-m/raises-silo-height-m-to-14-5: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-silo-height-m/raises-silo-height-m-to-14-5: moving silo_height_m from 10.0 to 14.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-silo-height-m/raises-silo-height-m-to-14-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-silo-height-m` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `siloHeightM` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `siloRadiusM`.
#[semio_framework_async_macros::async_test]
fn change_silo_height_m_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().silo_height_m, Some(14.5), "change-silo-height-m/raises-silo-height-m-to-14-5: the diff must set silo_height_m to 14.5");
    assert!(outcome.diff().artifact.is_none(), "change-silo-height-m/raises-silo-height-m-to-14-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().silo_radius_m.is_none(), "change-silo-height-m/raises-silo-height-m-to-14-5: change-silo-height-m must leave silo_radius_m untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-silo-height-m produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-height-m committed diff decodes");
    assert_eq!(produced, committed, "change-silo-height-m/raises-silo-height-m-to-14-5: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `siloHeightM` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-silo-height-m`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_silo_height_m_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-silo-height-m committed diff decodes");
    assert_eq!(decoded.silo_height_m, Some(14.5), "change-silo-height-m/raises-silo-height-m-to-14-5: the committed diff must carry silo_height_m at 14.5");
    assert!(decoded.selected_check_index.is_none(), "change-silo-height-m/raises-silo-height-m-to-14-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-silo-height-m committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-height-m committed diff reparses");
    assert_eq!(reencoded, original, "change-silo-height-m/raises-silo-height-m-to-14-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 10.0 to 14.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_silo_height_m_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-silo-height-m committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-silo-height-m committed diff applies to the before-snapshot");
    assert_eq!(produced.silo_height_m, 14.5, "change-silo-height-m/raises-silo-height-m-to-14-5: the committed diff must leave silo_height_m reading 14.5");
    assert_eq!(produced, expected_after(), "change-silo-height-m/raises-silo-height-m-to-14-5: the committed diff did not carry before to after");
}
