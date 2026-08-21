//! 🧪️ `change-bridge-sigma-c-mpa` fixture — `raises-bridge-sigma-c-mpa-to-15-75`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1992Diff.bridge_sigma_c_mpa` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.bridge_sigma_c_mpa == payload.new_bridge_sigma_c_mpa` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-bridge-sigma-c-mpa before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-bridge-sigma-c-mpa after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-bridge-sigma-c-mpa mutation decodes")
}

/// ▶️ `change-bridge-sigma-c-mpa` carries the committed before-snapshot to the committed after-snapshot by moving
/// `bridge_sigma_c_mpa` from 12.0 to 15.75, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_bridge_sigma_c_mpa_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-bridge-sigma-c-mpa applies to its committed before-snapshot");
    assert_eq!(applied.bridge_sigma_c_mpa, 15.75, "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: bridge_sigma_c_mpa must read 15.75 after the change");
    assert_eq!(applied, expected_after(), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: a real 12.0 to 15.75 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-bridge-sigma-c-mpa` is its own inverse partner: the inverse step restores `bridge_sigma_c_mpa` to its pre-change
/// 12.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_bridge_sigma_c_mpa_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-bridge-sigma-c-mpa applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-bridge-sigma-c-mpa step applies");
        restored = next;
    }
    assert_eq!(restored.bridge_sigma_c_mpa, 12.0, "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: the inverse must put bridge_sigma_c_mpa back to 12.0");
    assert_eq!(restored, base, "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeBridgeSigmaCMpa` payload are already canonical:
/// decode then encode is a fixed point, so `bridgeSigmaCMpa` and `newBridgeSigmaCMpa` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_bridge_sigma_c_mpa_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-bridge-sigma-c-mpa snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-bridge-sigma-c-mpa snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-bridge-sigma-c-mpa snapshot reparses");
        assert_eq!(reencoded, original, "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-bridge-sigma-c-mpa mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-bridge-sigma-c-mpa mutation reparses");
    assert_eq!(reencoded, original, "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: committed mutation JSON is not the canonical externally-tagged ChangeBridgeSigmaCMpa form carrying newBridgeSigmaCMpa");
}

/// 🎯️ The declared outcome holds: `change-bridge-sigma-c-mpa` at 15.75 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_bridge_sigma_c_mpa_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-bridge-sigma-c-mpa outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: moving bridge_sigma_c_mpa from 12.0 to 15.75 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-bridge-sigma-c-mpa` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `bridgeSigmaCMpa` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `bridgeDeltaSigmaSMpa`.
#[semio_framework_async_macros::async_test]
async fn change_bridge_sigma_c_mpa_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().bridge_sigma_c_mpa, Some(15.75), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: the diff must set bridge_sigma_c_mpa to 15.75");
    assert!(outcome.diff().artifact.is_none(), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().bridge_delta_sigma_s_mpa.is_none(), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: change-bridge-sigma-c-mpa must leave bridge_delta_sigma_s_mpa untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-bridge-sigma-c-mpa produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-bridge-sigma-c-mpa committed diff decodes");
    assert_eq!(produced, committed, "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `bridgeSigmaCMpa` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-bridge-sigma-c-mpa`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_bridge_sigma_c_mpa_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-bridge-sigma-c-mpa committed diff decodes");
    assert_eq!(decoded.bridge_sigma_c_mpa, Some(15.75), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: the committed diff must carry bridge_sigma_c_mpa at 15.75");
    assert!(decoded.selected_check_index.is_none(), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-bridge-sigma-c-mpa committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-bridge-sigma-c-mpa committed diff reparses");
    assert_eq!(reencoded, original, "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 12.0 to 15.75 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_bridge_sigma_c_mpa_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-bridge-sigma-c-mpa committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-bridge-sigma-c-mpa committed diff applies to the before-snapshot");
    assert_eq!(produced.bridge_sigma_c_mpa, 15.75, "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: the committed diff must leave bridge_sigma_c_mpa reading 15.75");
    assert_eq!(produced, expected_after(), "change-bridge-sigma-c-mpa/raises-bridge-sigma-c-mpa-to-15-75: the committed diff did not carry before to after");
}
