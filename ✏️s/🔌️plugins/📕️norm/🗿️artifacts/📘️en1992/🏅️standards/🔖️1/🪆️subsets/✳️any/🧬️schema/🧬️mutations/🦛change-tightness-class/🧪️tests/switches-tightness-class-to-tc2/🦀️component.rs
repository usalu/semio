//! 🧪️ `change-tightness-class` fixture — `switches-tightness-class-to-tc2`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1992Diff.tightness_class` and nothing else,
//! behind a `base.tightness_class == payload.new_tightness_class` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-tightness-class before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-tightness-class after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-tightness-class mutation decodes")
}

/// ▶️ `change-tightness-class` carries the committed before-snapshot to the committed after-snapshot by moving
/// `tightness_class` from Tc1 to Tc2, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_tightness_class_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-tightness-class applies to its committed before-snapshot");
    assert_eq!(applied.tightness_class, crate::artifacts::en1992::part_3::TightnessClass::Tc2, "change-tightness-class/switches-tightness-class-to-tc2: tightness_class must read Tc2 after the change");
    assert_eq!(applied, expected_after(), "change-tightness-class/switches-tightness-class-to-tc2: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-tightness-class/switches-tightness-class-to-tc2: a real Tc1 to Tc2 change must raise no `mutation.no-op` message");
}

/// ↩️ `change-tightness-class` is its own inverse partner: the inverse step restores `tightness_class` to its pre-change
/// Tc1 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_tightness_class_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-tightness-class applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-tightness-class/switches-tightness-class-to-tc2: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-tightness-class step applies");
        restored = next;
    }
    assert_eq!(restored.tightness_class, crate::artifacts::en1992::part_3::TightnessClass::Tc1, "change-tightness-class/switches-tightness-class-to-tc2: the inverse must put tightness_class back to Tc1");
    assert_eq!(restored, base, "change-tightness-class/switches-tightness-class-to-tc2: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeTightnessClass` payload are already canonical:
/// decode then encode is a fixed point, so `tightnessClass` and `newTightnessClass` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_tightness_class_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-tightness-class snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-tightness-class snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-tightness-class snapshot reparses");
        assert_eq!(reencoded, original, "change-tightness-class/switches-tightness-class-to-tc2: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-tightness-class mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-tightness-class mutation reparses");
    assert_eq!(reencoded, original, "change-tightness-class/switches-tightness-class-to-tc2: committed mutation JSON is not the canonical externally-tagged ChangeTightnessClass form carrying newTightnessClass");
}

/// 🎯️ The declared outcome holds: `change-tightness-class` at Tc2 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_tightness_class_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-tightness-class outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-tightness-class/switches-tightness-class-to-tc2: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-tightness-class/switches-tightness-class-to-tc2: moving tightness_class from Tc1 to Tc2 must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-tightness-class/switches-tightness-class-to-tc2: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-tightness-class` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `tightnessClass` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `hdOverH`.
#[semio_framework_async_macros::async_test]
async fn change_tightness_class_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().tightness_class, Some(crate::artifacts::en1992::part_3::TightnessClass::Tc2), "change-tightness-class/switches-tightness-class-to-tc2: the diff must set tightness_class to Tc2");
    assert!(outcome.diff().artifact.is_none(), "change-tightness-class/switches-tightness-class-to-tc2: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().hd_over_h.is_none(), "change-tightness-class/switches-tightness-class-to-tc2: change-tightness-class must leave hd_over_h untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-tightness-class produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-tightness-class committed diff decodes");
    assert_eq!(produced, committed, "change-tightness-class/switches-tightness-class-to-tc2: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `tightnessClass` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-tightness-class`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_tightness_class_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-tightness-class committed diff decodes");
    assert_eq!(decoded.tightness_class, Some(crate::artifacts::en1992::part_3::TightnessClass::Tc2), "change-tightness-class/switches-tightness-class-to-tc2: the committed diff must carry tightness_class at Tc2");
    assert!(decoded.selected_check_index.is_none(), "change-tightness-class/switches-tightness-class-to-tc2: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-tightness-class committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-tightness-class committed diff reparses");
    assert_eq!(reencoded, original, "change-tightness-class/switches-tightness-class-to-tc2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the Tc1 to Tc2 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_tightness_class_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-tightness-class committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-tightness-class committed diff applies to the before-snapshot");
    assert_eq!(produced.tightness_class, crate::artifacts::en1992::part_3::TightnessClass::Tc2, "change-tightness-class/switches-tightness-class-to-tc2: the committed diff must leave tightness_class reading Tc2");
    assert_eq!(produced, expected_after(), "change-tightness-class/switches-tightness-class-to-tc2: the committed diff did not carry before to after");
}
