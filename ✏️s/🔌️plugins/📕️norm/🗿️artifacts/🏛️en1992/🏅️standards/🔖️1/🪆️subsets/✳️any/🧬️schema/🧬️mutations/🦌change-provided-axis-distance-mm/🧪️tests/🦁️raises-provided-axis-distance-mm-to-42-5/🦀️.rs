//! 🧪️ `change-provided-axis-distance-mm` fixture — `🦁️raises-provided-axis-distance-mm-to-42-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1992Diff.provided_axis_distance_mm` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.provided_axis_distance_mm == payload.new_provided_axis_distance_mm` `mutation.no-op` guard.
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1992Snapshot {
    serde_json::from_str(BEFORE).expect("change-provided-axis-distance-mm before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-provided-axis-distance-mm after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-provided-axis-distance-mm mutation decodes")
}

/// ▶️ `change-provided-axis-distance-mm` carries the committed before-snapshot to the committed after-snapshot by moving
/// `provided_axis_distance_mm` from 30.0 to 42.5, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
fn change_provided_axis_distance_mm_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-provided-axis-distance-mm applies to its committed before-snapshot");
    assert_eq!(applied.provided_axis_distance_mm, 42.5, "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: provided_axis_distance_mm must read 42.5 after the change");
    assert_eq!(applied, expected_after(), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: a real 30.0 to 42.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-provided-axis-distance-mm` is its own inverse partner: the inverse step restores `provided_axis_distance_mm` to its pre-change
/// 30.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_provided_axis_distance_mm_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-provided-axis-distance-mm applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-provided-axis-distance-mm step applies");
        restored = next;
    }
    assert_eq!(restored.provided_axis_distance_mm, 30.0, "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: the inverse must put provided_axis_distance_mm back to 30.0");
    assert_eq!(restored, base, "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeProvidedAxisDistanceMm` payload are already canonical:
/// decode then encode is a fixed point, so `providedAxisDistanceMm` and `newProvidedAxisDistanceMm` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_provided_axis_distance_mm_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-provided-axis-distance-mm snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-provided-axis-distance-mm snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-provided-axis-distance-mm snapshot reparses");
        assert_eq!(reencoded, original, "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-provided-axis-distance-mm mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-provided-axis-distance-mm mutation reparses");
    assert_eq!(reencoded, original, "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: committed mutation JSON is not the canonical externally-tagged ChangeProvidedAxisDistanceMm form carrying newProvidedAxisDistanceMm");
}

/// 🎯️ The declared outcome holds: `change-provided-axis-distance-mm` at 42.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_provided_axis_distance_mm_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-provided-axis-distance-mm outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(
        outcome.messages().is_empty(),
        "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: moving provided_axis_distance_mm from 30.0 to 42.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message"
    );
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-provided-axis-distance-mm` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `providedAxisDistanceMm` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `bridgeSigmaCMpa`.
#[semio_framework_async_macros::async_test]
fn change_provided_axis_distance_mm_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().provided_axis_distance_mm, Some(42.5), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: the diff must set provided_axis_distance_mm to 42.5");
    assert!(outcome.diff().artifact.is_none(), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().bridge_sigma_c_mpa.is_none(), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: change-provided-axis-distance-mm must leave bridge_sigma_c_mpa untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-provided-axis-distance-mm produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-provided-axis-distance-mm committed diff decodes");
    assert_eq!(produced, committed, "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `providedAxisDistanceMm` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-provided-axis-distance-mm`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_provided_axis_distance_mm_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-provided-axis-distance-mm committed diff decodes");
    assert_eq!(decoded.provided_axis_distance_mm, Some(42.5), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: the committed diff must carry provided_axis_distance_mm at 42.5");
    assert!(decoded.selected_check_index.is_none(), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-provided-axis-distance-mm committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-provided-axis-distance-mm committed diff reparses");
    assert_eq!(reencoded, original, "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 30.0 to 42.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_provided_axis_distance_mm_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-provided-axis-distance-mm committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-provided-axis-distance-mm committed diff applies to the before-snapshot");
    assert_eq!(produced.provided_axis_distance_mm, 42.5, "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: the committed diff must leave provided_axis_distance_mm reading 42.5");
    assert_eq!(produced, expected_after(), "change-provided-axis-distance-mm/raises-provided-axis-distance-mm-to-42-5: the committed diff did not carry before to after");
}
