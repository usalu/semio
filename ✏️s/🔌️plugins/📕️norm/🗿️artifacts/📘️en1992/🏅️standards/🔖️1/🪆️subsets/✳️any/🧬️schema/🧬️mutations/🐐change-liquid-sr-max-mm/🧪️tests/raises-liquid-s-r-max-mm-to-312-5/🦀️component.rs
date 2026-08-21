//! 🧪️ `change-liquid-sr-max-mm` fixture — `raises-liquid-s-r-max-mm-to-312-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1992Diff.liquid_s_r_max_mm` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.liquid_s_r_max_mm == payload.new_liquid_s_r_max_mm` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-liquid-sr-max-mm before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-liquid-sr-max-mm after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-liquid-sr-max-mm mutation decodes")
}

/// ▶️ `change-liquid-sr-max-mm` carries the committed before-snapshot to the committed after-snapshot by moving
/// `liquid_s_r_max_mm` from 250.0 to 312.5, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_liquid_sr_max_mm_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-liquid-sr-max-mm applies to its committed before-snapshot");
    assert_eq!(applied.liquid_s_r_max_mm, 312.5, "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: liquid_s_r_max_mm must read 312.5 after the change");
    assert_eq!(applied, expected_after(), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: a real 250.0 to 312.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-liquid-sr-max-mm` is its own inverse partner: the inverse step restores `liquid_s_r_max_mm` to its pre-change
/// 250.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_liquid_sr_max_mm_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-liquid-sr-max-mm applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-liquid-sr-max-mm step applies");
        restored = next;
    }
    assert_eq!(restored.liquid_s_r_max_mm, 250.0, "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: the inverse must put liquid_s_r_max_mm back to 250.0");
    assert_eq!(restored, base, "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeLiquidSRMaxMm` payload are already canonical:
/// decode then encode is a fixed point, so `liquidSRMaxMm` and `newLiquidSRMaxMm` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_liquid_sr_max_mm_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-liquid-sr-max-mm snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-liquid-sr-max-mm snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-liquid-sr-max-mm snapshot reparses");
        assert_eq!(reencoded, original, "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-liquid-sr-max-mm mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-liquid-sr-max-mm mutation reparses");
    assert_eq!(reencoded, original, "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: committed mutation JSON is not the canonical externally-tagged ChangeLiquidSRMaxMm form carrying newLiquidSRMaxMm");
}

/// 🎯️ The declared outcome holds: `change-liquid-sr-max-mm` at 312.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_liquid_sr_max_mm_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-liquid-sr-max-mm outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: moving liquid_s_r_max_mm from 250.0 to 312.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-liquid-sr-max-mm` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `liquidSRMaxMm` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `anchorHEfMm`.
#[semio_framework_async_macros::async_test]
async fn change_liquid_sr_max_mm_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().liquid_s_r_max_mm, Some(312.5), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: the diff must set liquid_s_r_max_mm to 312.5");
    assert!(outcome.diff().artifact.is_none(), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().anchor_h_ef_mm.is_none(), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: change-liquid-sr-max-mm must leave anchor_h_ef_mm untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-liquid-sr-max-mm produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-liquid-sr-max-mm committed diff decodes");
    assert_eq!(produced, committed, "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `liquidSRMaxMm` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-liquid-sr-max-mm`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_liquid_sr_max_mm_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-liquid-sr-max-mm committed diff decodes");
    assert_eq!(decoded.liquid_s_r_max_mm, Some(312.5), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: the committed diff must carry liquid_s_r_max_mm at 312.5");
    assert!(decoded.selected_check_index.is_none(), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-liquid-sr-max-mm committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-liquid-sr-max-mm committed diff reparses");
    assert_eq!(reencoded, original, "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 250.0 to 312.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_liquid_sr_max_mm_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-liquid-sr-max-mm committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-liquid-sr-max-mm committed diff applies to the before-snapshot");
    assert_eq!(produced.liquid_s_r_max_mm, 312.5, "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: the committed diff must leave liquid_s_r_max_mm reading 312.5");
    assert_eq!(produced, expected_after(), "change-liquid-sr-max-mm/raises-liquid-s-r-max-mm-to-312-5: the committed diff did not carry before to after");
}
