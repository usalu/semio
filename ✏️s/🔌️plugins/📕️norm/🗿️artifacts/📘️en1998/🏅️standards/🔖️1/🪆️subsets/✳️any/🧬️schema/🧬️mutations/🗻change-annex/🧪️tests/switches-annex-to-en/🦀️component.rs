//! 🧪️ `change-annex` fixture — `switches-annex-to-en`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.annex` and nothing else,
//! behind a `base.annex == payload.new_annex` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-annex before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-annex after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-annex mutation decodes")
}

/// ▶️ `change-annex` carries the committed before-snapshot to the committed after-snapshot by moving
/// `annex` from de to en, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_annex_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-annex applies to its committed before-snapshot");
    assert_eq!(applied.annex, "en", "change-annex/switches-annex-to-en: annex must read en after the change");
    assert_eq!(applied, expected_after(), "change-annex/switches-annex-to-en: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-annex/switches-annex-to-en: a real de to en change must raise no `mutation.no-op` message");
}

/// ↩️ `change-annex` is its own inverse partner: the inverse step restores `annex` to its pre-change
/// de and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_annex_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-annex applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-annex/switches-annex-to-en: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-annex step applies");
        restored = next;
    }
    assert_eq!(restored.annex, "de", "change-annex/switches-annex-to-en: the inverse must put annex back to de");
    assert_eq!(restored, base, "change-annex/switches-annex-to-en: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeAnnex` payload are already canonical:
/// decode then encode is a fixed point, so `annex` and `newAnnex` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_annex_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-annex snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-annex snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-annex snapshot reparses");
        assert_eq!(reencoded, original, "change-annex/switches-annex-to-en: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-annex mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-annex mutation reparses");
    assert_eq!(reencoded, original, "change-annex/switches-annex-to-en: committed mutation JSON is not the canonical externally-tagged ChangeAnnex form carrying newAnnex");
}

/// 🎯️ The declared outcome holds: `change-annex` at en is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_annex_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-annex outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-annex/switches-annex-to-en: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-annex/switches-annex-to-en: moving annex from de to en must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-annex/switches-annex-to-en: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-annex` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `annex` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `enAGr`.
#[semio_framework_async_macros::async_test]
fn change_annex_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().annex.as_deref(), Some("en"), "change-annex/switches-annex-to-en: the diff must set annex to en");
    assert!(outcome.diff().artifact.is_none(), "change-annex/switches-annex-to-en: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().en_a_gr.is_none(), "change-annex/switches-annex-to-en: change-annex must leave en_a_gr untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-annex produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-annex committed diff decodes");
    assert_eq!(produced, committed, "change-annex/switches-annex-to-en: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `annex` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-annex`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_annex_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-annex committed diff decodes");
    assert_eq!(decoded.annex.as_deref(), Some("en"), "change-annex/switches-annex-to-en: the committed diff must carry annex at en");
    assert!(decoded.selected_check_index.is_none(), "change-annex/switches-annex-to-en: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-annex committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-annex committed diff reparses");
    assert_eq!(reencoded, original, "change-annex/switches-annex-to-en: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the de to en delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_annex_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-annex committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-annex committed diff applies to the before-snapshot");
    assert_eq!(produced.annex, "en", "change-annex/switches-annex-to-en: the committed diff must leave annex reading en");
    assert_eq!(produced, expected_after(), "change-annex/switches-annex-to-en: the committed diff did not carry before to after");
}
