//! 🧪️ `change-multiple-resisting-systems` fixture — `turns-multiple-resisting-systems-off`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.multiple_resisting_systems` and nothing else,
//! behind a `base.multiple_resisting_systems == payload.new_multiple_resisting_systems` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-multiple-resisting-systems before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-multiple-resisting-systems after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-multiple-resisting-systems mutation decodes")
}

/// ▶️ `change-multiple-resisting-systems` carries the committed before-snapshot to the committed after-snapshot by moving
/// `multiple_resisting_systems` from true to false, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_multiple_resisting_systems_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-multiple-resisting-systems applies to its committed before-snapshot");
    assert!(!applied.multiple_resisting_systems, "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: multiple_resisting_systems must read false after the change");
    assert_eq!(applied, expected_after(), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: a real true to false change must raise no `mutation.no-op` message");
}

/// ↩️ `change-multiple-resisting-systems` is its own inverse partner: the inverse step restores `multiple_resisting_systems` to its pre-change
/// true and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_multiple_resisting_systems_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-multiple-resisting-systems applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-multiple-resisting-systems step applies");
        restored = next;
    }
    assert!(restored.multiple_resisting_systems, "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: the inverse must put multiple_resisting_systems back to true");
    assert_eq!(restored, base, "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeMultipleResistingSystems` payload are already canonical:
/// decode then encode is a fixed point, so `multipleResistingSystems` and `newMultipleResistingSystems` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_multiple_resisting_systems_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-multiple-resisting-systems snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-multiple-resisting-systems snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-multiple-resisting-systems snapshot reparses");
        assert_eq!(reencoded, original, "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-multiple-resisting-systems mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-multiple-resisting-systems mutation reparses");
    assert_eq!(reencoded, original, "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: committed mutation JSON is not the canonical externally-tagged ChangeMultipleResistingSystems form carrying newMultipleResistingSystems");
}

/// 🎯️ The declared outcome holds: `change-multiple-resisting-systems` at false is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_multiple_resisting_systems_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-multiple-resisting-systems outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: moving multiple_resisting_systems from true to false must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-multiple-resisting-systems` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `multipleResistingSystems` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `annex`.
#[semio_framework_async_macros::async_test]
fn change_multiple_resisting_systems_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().multiple_resisting_systems, Some(false), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: the diff must set multiple_resisting_systems to false");
    assert!(outcome.diff().artifact.is_none(), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().annex.is_none(), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: change-multiple-resisting-systems must leave annex untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-multiple-resisting-systems produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-multiple-resisting-systems committed diff decodes");
    assert_eq!(produced, committed, "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `multipleResistingSystems` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-multiple-resisting-systems`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_multiple_resisting_systems_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-multiple-resisting-systems committed diff decodes");
    assert_eq!(decoded.multiple_resisting_systems, Some(false), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: the committed diff must carry multiple_resisting_systems at false");
    assert!(decoded.selected_check_index.is_none(), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-multiple-resisting-systems committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-multiple-resisting-systems committed diff reparses");
    assert_eq!(reencoded, original, "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the true to false delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_multiple_resisting_systems_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-multiple-resisting-systems committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-multiple-resisting-systems committed diff applies to the before-snapshot");
    assert!(!produced.multiple_resisting_systems, "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: the committed diff must leave multiple_resisting_systems reading false");
    assert_eq!(produced, expected_after(), "change-multiple-resisting-systems/turns-multiple-resisting-systems-off: the committed diff did not carry before to after");
}
