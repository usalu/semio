//! 🧪️ `change-drift-mm` fixture — `raises-drift-mm-to-33-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.drift_mm` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.drift_mm == payload.new_drift_mm` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-drift-mm before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-drift-mm after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-drift-mm mutation decodes")
}

/// ▶️ `change-drift-mm` carries the committed before-snapshot to the committed after-snapshot by moving
/// `drift_mm` from 20.0 to 33.5, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_drift_mm_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-drift-mm applies to its committed before-snapshot");
    assert_eq!(applied.drift_mm, 33.5, "change-drift-mm/raises-drift-mm-to-33-5: drift_mm must read 33.5 after the change");
    assert_eq!(applied, expected_after(), "change-drift-mm/raises-drift-mm-to-33-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-drift-mm/raises-drift-mm-to-33-5: a real 20.0 to 33.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-drift-mm` is its own inverse partner: the inverse step restores `drift_mm` to its pre-change
/// 20.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_drift_mm_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-drift-mm applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-drift-mm/raises-drift-mm-to-33-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-drift-mm step applies");
        restored = next;
    }
    assert_eq!(restored.drift_mm, 20.0, "change-drift-mm/raises-drift-mm-to-33-5: the inverse must put drift_mm back to 20.0");
    assert_eq!(restored, base, "change-drift-mm/raises-drift-mm-to-33-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeDriftMm` payload are already canonical:
/// decode then encode is a fixed point, so `driftMm` and `newDriftMm` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_drift_mm_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-drift-mm snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-drift-mm snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-drift-mm snapshot reparses");
        assert_eq!(reencoded, original, "change-drift-mm/raises-drift-mm-to-33-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-drift-mm mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-drift-mm mutation reparses");
    assert_eq!(reencoded, original, "change-drift-mm/raises-drift-mm-to-33-5: committed mutation JSON is not the canonical externally-tagged ChangeDriftMm form carrying newDriftMm");
}

/// 🎯️ The declared outcome holds: `change-drift-mm` at 33.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_drift_mm_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-drift-mm outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-drift-mm/raises-drift-mm-to-33-5: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-drift-mm/raises-drift-mm-to-33-5: moving drift_mm from 20.0 to 33.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-drift-mm/raises-drift-mm-to-33-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-drift-mm` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `driftMm` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `heightM`.
#[semio_framework_async_macros::async_test]
async fn change_drift_mm_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().drift_mm, Some(33.5), "change-drift-mm/raises-drift-mm-to-33-5: the diff must set drift_mm to 33.5");
    assert!(outcome.diff().artifact.is_none(), "change-drift-mm/raises-drift-mm-to-33-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().height_m.is_none(), "change-drift-mm/raises-drift-mm-to-33-5: change-drift-mm must leave height_m untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-drift-mm produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-drift-mm committed diff decodes");
    assert_eq!(produced, committed, "change-drift-mm/raises-drift-mm-to-33-5: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `driftMm` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-drift-mm`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_drift_mm_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-drift-mm committed diff decodes");
    assert_eq!(decoded.drift_mm, Some(33.5), "change-drift-mm/raises-drift-mm-to-33-5: the committed diff must carry drift_mm at 33.5");
    assert!(decoded.selected_check_index.is_none(), "change-drift-mm/raises-drift-mm-to-33-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-drift-mm committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-drift-mm committed diff reparses");
    assert_eq!(reencoded, original, "change-drift-mm/raises-drift-mm-to-33-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 20.0 to 33.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_drift_mm_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-drift-mm committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-drift-mm committed diff applies to the before-snapshot");
    assert_eq!(produced.drift_mm, 33.5, "change-drift-mm/raises-drift-mm-to-33-5: the committed diff must leave drift_mm reading 33.5");
    assert_eq!(produced, expected_after(), "change-drift-mm/raises-drift-mm-to-33-5: the committed diff did not carry before to after");
}
