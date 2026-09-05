//! 🧪️ `change-liquid-rho-p-eff` fixture — `🖱️raises-liquid-rho-p-eff-to-0-0078125`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1992Diff.liquid_rho_p_eff` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.liquid_rho_p_eff == payload.new_liquid_rho_p_eff` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-liquid-rho-p-eff before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-liquid-rho-p-eff after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-liquid-rho-p-eff mutation decodes")
}

/// ▶️ `change-liquid-rho-p-eff` carries the committed before-snapshot to the committed after-snapshot by moving
/// `liquid_rho_p_eff` from 0.01 to 0.0078125, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
fn change_liquid_rho_p_eff_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-liquid-rho-p-eff applies to its committed before-snapshot");
    assert_eq!(applied.liquid_rho_p_eff, 0.0078125, "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: liquid_rho_p_eff must read 0.0078125 after the change");
    assert_eq!(applied, expected_after(), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: a real 0.01 to 0.0078125 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-liquid-rho-p-eff` is its own inverse partner: the inverse step restores `liquid_rho_p_eff` to its pre-change
/// 0.01 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_liquid_rho_p_eff_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-liquid-rho-p-eff applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-liquid-rho-p-eff step applies");
        restored = next;
    }
    assert_eq!(restored.liquid_rho_p_eff, 0.01, "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: the inverse must put liquid_rho_p_eff back to 0.01");
    assert_eq!(restored, base, "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeLiquidRhoPEff` payload are already canonical:
/// decode then encode is a fixed point, so `liquidRhoPEff` and `newLiquidRhoPEff` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_liquid_rho_p_eff_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-liquid-rho-p-eff snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-liquid-rho-p-eff snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-liquid-rho-p-eff snapshot reparses");
        assert_eq!(reencoded, original, "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-liquid-rho-p-eff mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-liquid-rho-p-eff mutation reparses");
    assert_eq!(reencoded, original, "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: committed mutation JSON is not the canonical externally-tagged ChangeLiquidRhoPEff form carrying newLiquidRhoPEff");
}

/// 🎯️ The declared outcome holds: `change-liquid-rho-p-eff` at 0.0078125 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_liquid_rho_p_eff_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-liquid-rho-p-eff outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: moving liquid_rho_p_eff from 0.01 to 0.0078125 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-liquid-rho-p-eff` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `liquidRhoPEff` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `liquidFCtEffMpa`.
#[semio_framework_async_macros::async_test]
fn change_liquid_rho_p_eff_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().liquid_rho_p_eff, Some(0.0078125), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: the diff must set liquid_rho_p_eff to 0.0078125");
    assert!(outcome.diff().artifact.is_none(), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().liquid_f_ct_eff_mpa.is_none(), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: change-liquid-rho-p-eff must leave liquid_f_ct_eff_mpa untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-liquid-rho-p-eff produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-liquid-rho-p-eff committed diff decodes");
    assert_eq!(produced, committed, "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `liquidRhoPEff` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-liquid-rho-p-eff`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_liquid_rho_p_eff_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-liquid-rho-p-eff committed diff decodes");
    assert_eq!(decoded.liquid_rho_p_eff, Some(0.0078125), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: the committed diff must carry liquid_rho_p_eff at 0.0078125");
    assert!(decoded.selected_check_index.is_none(), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-liquid-rho-p-eff committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-liquid-rho-p-eff committed diff reparses");
    assert_eq!(reencoded, original, "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 0.01 to 0.0078125 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_liquid_rho_p_eff_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-liquid-rho-p-eff committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-liquid-rho-p-eff committed diff applies to the before-snapshot");
    assert_eq!(produced.liquid_rho_p_eff, 0.0078125, "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: the committed diff must leave liquid_rho_p_eff reading 0.0078125");
    assert_eq!(produced, expected_after(), "change-liquid-rho-p-eff/raises-liquid-rho-p-eff-to-0-0078125: the committed diff did not carry before to after");
}
