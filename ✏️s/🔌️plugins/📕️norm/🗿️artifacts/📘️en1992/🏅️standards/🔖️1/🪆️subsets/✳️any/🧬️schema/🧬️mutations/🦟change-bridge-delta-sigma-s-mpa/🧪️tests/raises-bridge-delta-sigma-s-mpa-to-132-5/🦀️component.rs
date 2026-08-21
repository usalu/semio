//! 🧪️ `change-bridge-delta-sigma-s-mpa` fixture — `raises-bridge-delta-sigma-s-mpa-to-132-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1992Diff.bridge_delta_sigma_s_mpa` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.bridge_delta_sigma_s_mpa == payload.new_bridge_delta_sigma_s_mpa` `mutation.no-op` guard.
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1992Snapshot {
    serde_json::from_str(BEFORE).expect("change-bridge-delta-sigma-s-mpa before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-bridge-delta-sigma-s-mpa after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-bridge-delta-sigma-s-mpa mutation decodes")
}

/// ▶️ `change-bridge-delta-sigma-s-mpa` carries the committed before-snapshot to the committed after-snapshot by moving
/// `bridge_delta_sigma_s_mpa` from 100.0 to 132.5, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_bridge_delta_sigma_s_mpa_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-bridge-delta-sigma-s-mpa applies to its committed before-snapshot");
    assert_eq!(applied.bridge_delta_sigma_s_mpa, 132.5, "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: bridge_delta_sigma_s_mpa must read 132.5 after the change");
    assert_eq!(applied, expected_after(), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: a real 100.0 to 132.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-bridge-delta-sigma-s-mpa` is its own inverse partner: the inverse step restores `bridge_delta_sigma_s_mpa` to its pre-change
/// 100.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_bridge_delta_sigma_s_mpa_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-bridge-delta-sigma-s-mpa applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-bridge-delta-sigma-s-mpa step applies");
        restored = next;
    }
    assert_eq!(restored.bridge_delta_sigma_s_mpa, 100.0, "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: the inverse must put bridge_delta_sigma_s_mpa back to 100.0");
    assert_eq!(restored, base, "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeBridgeDeltaSigmaSMpa` payload are already canonical:
/// decode then encode is a fixed point, so `bridgeDeltaSigmaSMpa` and `newBridgeDeltaSigmaSMpa` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_bridge_delta_sigma_s_mpa_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-bridge-delta-sigma-s-mpa snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-bridge-delta-sigma-s-mpa snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-bridge-delta-sigma-s-mpa snapshot reparses");
        assert_eq!(reencoded, original, "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-bridge-delta-sigma-s-mpa mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-bridge-delta-sigma-s-mpa mutation reparses");
    assert_eq!(reencoded, original, "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: committed mutation JSON is not the canonical externally-tagged ChangeBridgeDeltaSigmaSMpa form carrying newBridgeDeltaSigmaSMpa");
}

/// 🎯️ The declared outcome holds: `change-bridge-delta-sigma-s-mpa` at 132.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_bridge_delta_sigma_s_mpa_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-bridge-delta-sigma-s-mpa outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: moving bridge_delta_sigma_s_mpa from 100.0 to 132.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-bridge-delta-sigma-s-mpa` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `bridgeDeltaSigmaSMpa` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `tightnessClass`.
#[semio_framework_async_macros::async_test]
async fn change_bridge_delta_sigma_s_mpa_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().bridge_delta_sigma_s_mpa, Some(132.5), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: the diff must set bridge_delta_sigma_s_mpa to 132.5");
    assert!(outcome.diff().artifact.is_none(), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().tightness_class.is_none(), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: change-bridge-delta-sigma-s-mpa must leave tightness_class untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-bridge-delta-sigma-s-mpa produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-bridge-delta-sigma-s-mpa committed diff decodes");
    assert_eq!(produced, committed, "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `bridgeDeltaSigmaSMpa` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-bridge-delta-sigma-s-mpa`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_bridge_delta_sigma_s_mpa_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-bridge-delta-sigma-s-mpa committed diff decodes");
    assert_eq!(decoded.bridge_delta_sigma_s_mpa, Some(132.5), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: the committed diff must carry bridge_delta_sigma_s_mpa at 132.5");
    assert!(decoded.selected_check_index.is_none(), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-bridge-delta-sigma-s-mpa committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-bridge-delta-sigma-s-mpa committed diff reparses");
    assert_eq!(reencoded, original, "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 100.0 to 132.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_bridge_delta_sigma_s_mpa_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-bridge-delta-sigma-s-mpa committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-bridge-delta-sigma-s-mpa committed diff applies to the before-snapshot");
    assert_eq!(produced.bridge_delta_sigma_s_mpa, 132.5, "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: the committed diff must leave bridge_delta_sigma_s_mpa reading 132.5");
    assert_eq!(produced, expected_after(), "change-bridge-delta-sigma-s-mpa/raises-bridge-delta-sigma-s-mpa-to-132-5: the committed diff did not carry before to after");
}
