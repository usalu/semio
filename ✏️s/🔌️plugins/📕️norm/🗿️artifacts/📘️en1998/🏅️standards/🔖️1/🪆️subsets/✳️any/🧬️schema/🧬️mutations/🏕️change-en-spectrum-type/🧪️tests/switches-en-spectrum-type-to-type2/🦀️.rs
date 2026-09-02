//! 🧪️ `change-en-spectrum-type` fixture — `switches-en-spectrum-type-to-type2`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.en_spectrum_type` and nothing else,
//! behind a `base.en_spectrum_type == payload.new_en_spectrum_type` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-en-spectrum-type before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-en-spectrum-type after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-en-spectrum-type mutation decodes")
}

/// ▶️ `change-en-spectrum-type` carries the committed before-snapshot to the committed after-snapshot by moving
/// `en_spectrum_type` from type1 to type2, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_en_spectrum_type_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-en-spectrum-type applies to its committed before-snapshot");
    assert_eq!(applied.en_spectrum_type, "type2", "change-en-spectrum-type/switches-en-spectrum-type-to-type2: en_spectrum_type must read type2 after the change");
    assert_eq!(applied, expected_after(), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: a real type1 to type2 change must raise no `mutation.no-op` message");
}

/// ↩️ `change-en-spectrum-type` is its own inverse partner: the inverse step restores `en_spectrum_type` to its pre-change
/// type1 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_en_spectrum_type_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-en-spectrum-type applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-en-spectrum-type/switches-en-spectrum-type-to-type2: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-en-spectrum-type step applies");
        restored = next;
    }
    assert_eq!(restored.en_spectrum_type, "type1", "change-en-spectrum-type/switches-en-spectrum-type-to-type2: the inverse must put en_spectrum_type back to type1");
    assert_eq!(restored, base, "change-en-spectrum-type/switches-en-spectrum-type-to-type2: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeEnSpectrumType` payload are already canonical:
/// decode then encode is a fixed point, so `enSpectrumType` and `newEnSpectrumType` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_en_spectrum_type_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-en-spectrum-type snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-en-spectrum-type snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-en-spectrum-type snapshot reparses");
        assert_eq!(reencoded, original, "change-en-spectrum-type/switches-en-spectrum-type-to-type2: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-en-spectrum-type mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-en-spectrum-type mutation reparses");
    assert_eq!(reencoded, original, "change-en-spectrum-type/switches-en-spectrum-type-to-type2: committed mutation JSON is not the canonical externally-tagged ChangeEnSpectrumType form carrying newEnSpectrumType");
}

/// 🎯️ The declared outcome holds: `change-en-spectrum-type` at type2 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_en_spectrum_type_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-en-spectrum-type outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: moving en_spectrum_type from type1 to type2 must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-en-spectrum-type` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `enSpectrumType` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `periodRatio`.
#[semio_framework_async_macros::async_test]
fn change_en_spectrum_type_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().en_spectrum_type.as_deref(), Some("type2"), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: the diff must set en_spectrum_type to type2");
    assert!(outcome.diff().artifact.is_none(), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().period_ratio.is_none(), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: change-en-spectrum-type must leave period_ratio untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-en-spectrum-type produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-en-spectrum-type committed diff decodes");
    assert_eq!(produced, committed, "change-en-spectrum-type/switches-en-spectrum-type-to-type2: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `enSpectrumType` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-en-spectrum-type`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_en_spectrum_type_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-en-spectrum-type committed diff decodes");
    assert_eq!(decoded.en_spectrum_type.as_deref(), Some("type2"), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: the committed diff must carry en_spectrum_type at type2");
    assert!(decoded.selected_check_index.is_none(), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-en-spectrum-type committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-en-spectrum-type committed diff reparses");
    assert_eq!(reencoded, original, "change-en-spectrum-type/switches-en-spectrum-type-to-type2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the type1 to type2 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_en_spectrum_type_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-en-spectrum-type committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-en-spectrum-type committed diff applies to the before-snapshot");
    assert_eq!(produced.en_spectrum_type, "type2", "change-en-spectrum-type/switches-en-spectrum-type-to-type2: the committed diff must leave en_spectrum_type reading type2");
    assert_eq!(produced, expected_after(), "change-en-spectrum-type/switches-en-spectrum-type-to-type2: the committed diff did not carry before to after");
}
