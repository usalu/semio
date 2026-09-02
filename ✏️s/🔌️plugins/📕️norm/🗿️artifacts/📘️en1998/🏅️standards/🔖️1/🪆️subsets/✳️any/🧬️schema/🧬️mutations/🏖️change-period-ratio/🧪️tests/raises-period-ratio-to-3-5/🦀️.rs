//! 🧪️ `change-period-ratio` fixture — `raises-period-ratio-to-3-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.period_ratio` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.period_ratio == payload.new_period_ratio` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-period-ratio before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-period-ratio after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-period-ratio mutation decodes")
}

/// ▶️ `change-period-ratio` carries the committed before-snapshot to the committed after-snapshot by moving
/// `period_ratio` from 2.0 to 3.5, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_period_ratio_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-period-ratio applies to its committed before-snapshot");
    assert_eq!(applied.period_ratio, 3.5, "change-period-ratio/raises-period-ratio-to-3-5: period_ratio must read 3.5 after the change");
    assert_eq!(applied, expected_after(), "change-period-ratio/raises-period-ratio-to-3-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-period-ratio/raises-period-ratio-to-3-5: a real 2.0 to 3.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-period-ratio` is its own inverse partner: the inverse step restores `period_ratio` to its pre-change
/// 2.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_period_ratio_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-period-ratio applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-period-ratio/raises-period-ratio-to-3-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-period-ratio step applies");
        restored = next;
    }
    assert_eq!(restored.period_ratio, 2.0, "change-period-ratio/raises-period-ratio-to-3-5: the inverse must put period_ratio back to 2.0");
    assert_eq!(restored, base, "change-period-ratio/raises-period-ratio-to-3-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangePeriodRatio` payload are already canonical:
/// decode then encode is a fixed point, so `periodRatio` and `newPeriodRatio` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_period_ratio_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-period-ratio snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-period-ratio snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-period-ratio snapshot reparses");
        assert_eq!(reencoded, original, "change-period-ratio/raises-period-ratio-to-3-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-period-ratio mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-period-ratio mutation reparses");
    assert_eq!(reencoded, original, "change-period-ratio/raises-period-ratio-to-3-5: committed mutation JSON is not the canonical externally-tagged ChangePeriodRatio form carrying newPeriodRatio");
}

/// 🎯️ The declared outcome holds: `change-period-ratio` at 3.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_period_ratio_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-period-ratio outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-period-ratio/raises-period-ratio-to-3-5: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-period-ratio/raises-period-ratio-to-3-5: moving period_ratio from 2.0 to 3.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-period-ratio/raises-period-ratio-to-3-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-period-ratio` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `periodRatio` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `bridgeVRdKn`.
#[semio_framework_async_macros::async_test]
fn change_period_ratio_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().period_ratio, Some(3.5), "change-period-ratio/raises-period-ratio-to-3-5: the diff must set period_ratio to 3.5");
    assert!(outcome.diff().artifact.is_none(), "change-period-ratio/raises-period-ratio-to-3-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().bridge_v_rd_kn.is_none(), "change-period-ratio/raises-period-ratio-to-3-5: change-period-ratio must leave bridge_v_rd_kn untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-period-ratio produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-period-ratio committed diff decodes");
    assert_eq!(produced, committed, "change-period-ratio/raises-period-ratio-to-3-5: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `periodRatio` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-period-ratio`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_period_ratio_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-period-ratio committed diff decodes");
    assert_eq!(decoded.period_ratio, Some(3.5), "change-period-ratio/raises-period-ratio-to-3-5: the committed diff must carry period_ratio at 3.5");
    assert!(decoded.selected_check_index.is_none(), "change-period-ratio/raises-period-ratio-to-3-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-period-ratio committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-period-ratio committed diff reparses");
    assert_eq!(reencoded, original, "change-period-ratio/raises-period-ratio-to-3-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 2.0 to 3.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_period_ratio_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-period-ratio committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-period-ratio committed diff applies to the before-snapshot");
    assert_eq!(produced.period_ratio, 3.5, "change-period-ratio/raises-period-ratio-to-3-5: the committed diff must leave period_ratio reading 3.5");
    assert_eq!(produced, expected_after(), "change-period-ratio/raises-period-ratio-to-3-5: the committed diff did not carry before to after");
}
