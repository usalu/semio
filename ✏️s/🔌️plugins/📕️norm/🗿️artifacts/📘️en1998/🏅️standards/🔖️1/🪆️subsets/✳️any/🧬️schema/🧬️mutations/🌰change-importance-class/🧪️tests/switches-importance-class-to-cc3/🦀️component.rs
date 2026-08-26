//! 🧪️ `change-importance-class` fixture — `switches-importance-class-to-cc3`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.importance_class` and nothing else,
//! behind a `base.importance_class == payload.new_importance_class` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-importance-class before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-importance-class after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-importance-class mutation decodes")
}

/// ▶️ `change-importance-class` carries the committed before-snapshot to the committed after-snapshot by moving
/// `importance_class` from cc2 to cc3, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_importance_class_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-importance-class applies to its committed before-snapshot");
    assert_eq!(applied.importance_class, "cc3", "change-importance-class/switches-importance-class-to-cc3: importance_class must read cc3 after the change");
    assert_eq!(applied, expected_after(), "change-importance-class/switches-importance-class-to-cc3: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-importance-class/switches-importance-class-to-cc3: a real cc2 to cc3 change must raise no `mutation.no-op` message");
}

/// ↩️ `change-importance-class` is its own inverse partner: the inverse step restores `importance_class` to its pre-change
/// cc2 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_importance_class_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-importance-class applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-importance-class/switches-importance-class-to-cc3: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-importance-class step applies");
        restored = next;
    }
    assert_eq!(restored.importance_class, "cc2", "change-importance-class/switches-importance-class-to-cc3: the inverse must put importance_class back to cc2");
    assert_eq!(restored, base, "change-importance-class/switches-importance-class-to-cc3: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeImportanceClass` payload are already canonical:
/// decode then encode is a fixed point, so `importanceClass` and `newImportanceClass` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_importance_class_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-importance-class snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-importance-class snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-importance-class snapshot reparses");
        assert_eq!(reencoded, original, "change-importance-class/switches-importance-class-to-cc3: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-importance-class mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-importance-class mutation reparses");
    assert_eq!(reencoded, original, "change-importance-class/switches-importance-class-to-cc3: committed mutation JSON is not the canonical externally-tagged ChangeImportanceClass form carrying newImportanceClass");
}

/// 🎯️ The declared outcome holds: `change-importance-class` at cc3 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_importance_class_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-importance-class outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-importance-class/switches-importance-class-to-cc3: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-importance-class/switches-importance-class-to-cc3: moving importance_class from cc2 to cc3 must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-importance-class/switches-importance-class-to-cc3: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-importance-class` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `importanceClass` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `structuralSystem`.
#[semio_framework_async_macros::async_test]
fn change_importance_class_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().importance_class.as_deref(), Some("cc3"), "change-importance-class/switches-importance-class-to-cc3: the diff must set importance_class to cc3");
    assert!(outcome.diff().artifact.is_none(), "change-importance-class/switches-importance-class-to-cc3: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().structural_system.is_none(), "change-importance-class/switches-importance-class-to-cc3: change-importance-class must leave structural_system untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-importance-class produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-importance-class committed diff decodes");
    assert_eq!(produced, committed, "change-importance-class/switches-importance-class-to-cc3: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `importanceClass` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-importance-class`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_importance_class_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-importance-class committed diff decodes");
    assert_eq!(decoded.importance_class.as_deref(), Some("cc3"), "change-importance-class/switches-importance-class-to-cc3: the committed diff must carry importance_class at cc3");
    assert!(decoded.selected_check_index.is_none(), "change-importance-class/switches-importance-class-to-cc3: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-importance-class committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-importance-class committed diff reparses");
    assert_eq!(reencoded, original, "change-importance-class/switches-importance-class-to-cc3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the cc2 to cc3 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_importance_class_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-importance-class committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-importance-class committed diff applies to the before-snapshot");
    assert_eq!(produced.importance_class, "cc3", "change-importance-class/switches-importance-class-to-cc3: the committed diff must leave importance_class reading cc3");
    assert_eq!(produced, expected_after(), "change-importance-class/switches-importance-class-to-cc3: the committed diff did not carry before to after");
}
