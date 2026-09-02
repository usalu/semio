//! 🧪️ `change-ground-type` fixture — `switches-ground-type-to-c`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.ground_type` and nothing else,
//! behind a `base.ground_type == payload.new_ground_type` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-ground-type before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-ground-type after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-ground-type mutation decodes")
}

/// ▶️ `change-ground-type` carries the committed before-snapshot to the committed after-snapshot by moving
/// `ground_type` from b to c, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_ground_type_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-ground-type applies to its committed before-snapshot");
    assert_eq!(applied.ground_type, "c", "change-ground-type/switches-ground-type-to-c: ground_type must read c after the change");
    assert_eq!(applied, expected_after(), "change-ground-type/switches-ground-type-to-c: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-ground-type/switches-ground-type-to-c: a real b to c change must raise no `mutation.no-op` message");
}

/// ↩️ `change-ground-type` is its own inverse partner: the inverse step restores `ground_type` to its pre-change
/// b and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_ground_type_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-ground-type applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-ground-type/switches-ground-type-to-c: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-ground-type step applies");
        restored = next;
    }
    assert_eq!(restored.ground_type, "b", "change-ground-type/switches-ground-type-to-c: the inverse must put ground_type back to b");
    assert_eq!(restored, base, "change-ground-type/switches-ground-type-to-c: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeGroundType` payload are already canonical:
/// decode then encode is a fixed point, so `groundType` and `newGroundType` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_ground_type_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-ground-type snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-ground-type snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-ground-type snapshot reparses");
        assert_eq!(reencoded, original, "change-ground-type/switches-ground-type-to-c: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-ground-type mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-ground-type mutation reparses");
    assert_eq!(reencoded, original, "change-ground-type/switches-ground-type-to-c: committed mutation JSON is not the canonical externally-tagged ChangeGroundType form carrying newGroundType");
}

/// 🎯️ The declared outcome holds: `change-ground-type` at c is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_ground_type_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-ground-type outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-ground-type/switches-ground-type-to-c: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-ground-type/switches-ground-type-to-c: moving ground_type from b to c must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-ground-type/switches-ground-type-to-c: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-ground-type` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `groundType` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `importanceClass`.
#[semio_framework_async_macros::async_test]
fn change_ground_type_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().ground_type.as_deref(), Some("c"), "change-ground-type/switches-ground-type-to-c: the diff must set ground_type to c");
    assert!(outcome.diff().artifact.is_none(), "change-ground-type/switches-ground-type-to-c: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().importance_class.is_none(), "change-ground-type/switches-ground-type-to-c: change-ground-type must leave importance_class untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-ground-type produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-ground-type committed diff decodes");
    assert_eq!(produced, committed, "change-ground-type/switches-ground-type-to-c: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `groundType` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-ground-type`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_ground_type_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-ground-type committed diff decodes");
    assert_eq!(decoded.ground_type.as_deref(), Some("c"), "change-ground-type/switches-ground-type-to-c: the committed diff must carry ground_type at c");
    assert!(decoded.selected_check_index.is_none(), "change-ground-type/switches-ground-type-to-c: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-ground-type committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-ground-type committed diff reparses");
    assert_eq!(reencoded, original, "change-ground-type/switches-ground-type-to-c: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the b to c delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_ground_type_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-ground-type committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-ground-type committed diff applies to the before-snapshot");
    assert_eq!(produced.ground_type, "c", "change-ground-type/switches-ground-type-to-c: the committed diff must leave ground_type reading c");
    assert_eq!(produced, expected_after(), "change-ground-type/switches-ground-type-to-c: the committed diff did not carry before to after");
}
