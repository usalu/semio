//! 🧪️ `change-structural-system` fixture — `switches-structural-system-to-wall-dcm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1998Diff.structural_system` and nothing else,
//! behind a `base.structural_system == payload.new_structural_system` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-structural-system before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-structural-system after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-structural-system mutation decodes")
}

/// ▶️ `change-structural-system` carries the committed before-snapshot to the committed after-snapshot by moving
/// `structural_system` from moment_frame_dch to wall_dcm, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
async fn change_structural_system_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-structural-system applies to its committed before-snapshot");
    assert_eq!(applied.structural_system, "wall_dcm", "change-structural-system/switches-structural-system-to-wall-dcm: structural_system must read wall_dcm after the change");
    assert_eq!(applied, expected_after(), "change-structural-system/switches-structural-system-to-wall-dcm: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-structural-system/switches-structural-system-to-wall-dcm: a real moment_frame_dch to wall_dcm change must raise no `mutation.no-op` message");
}

/// ↩️ `change-structural-system` is its own inverse partner: the inverse step restores `structural_system` to its pre-change
/// moment_frame_dch and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn change_structural_system_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-structural-system applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-structural-system/switches-structural-system-to-wall-dcm: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-structural-system step applies");
        restored = next;
    }
    assert_eq!(restored.structural_system, "moment_frame_dch", "change-structural-system/switches-structural-system-to-wall-dcm: the inverse must put structural_system back to moment_frame_dch");
    assert_eq!(restored, base, "change-structural-system/switches-structural-system-to-wall-dcm: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeStructuralSystem` payload are already canonical:
/// decode then encode is a fixed point, so `structuralSystem` and `newStructuralSystem` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn change_structural_system_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-structural-system snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-structural-system snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-structural-system snapshot reparses");
        assert_eq!(reencoded, original, "change-structural-system/switches-structural-system-to-wall-dcm: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-structural-system mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-structural-system mutation reparses");
    assert_eq!(reencoded, original, "change-structural-system/switches-structural-system-to-wall-dcm: committed mutation JSON is not the canonical externally-tagged ChangeStructuralSystem form carrying newStructuralSystem");
}

/// 🎯️ The declared outcome holds: `change-structural-system` at wall_dcm is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn change_structural_system_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-structural-system outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-structural-system/switches-structural-system-to-wall-dcm: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-structural-system/switches-structural-system-to-wall-dcm: moving structural_system from moment_frame_dch to wall_dcm must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-structural-system/switches-structural-system-to-wall-dcm: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-structural-system` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `structuralSystem` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `t1S`.
#[semio_framework_async_macros::async_test]
async fn change_structural_system_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().structural_system.as_deref(), Some("wall_dcm"), "change-structural-system/switches-structural-system-to-wall-dcm: the diff must set structural_system to wall_dcm");
    assert!(outcome.diff().artifact.is_none(), "change-structural-system/switches-structural-system-to-wall-dcm: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().t1_s.is_none(), "change-structural-system/switches-structural-system-to-wall-dcm: change-structural-system must leave t1_s untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-structural-system produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-structural-system committed diff decodes");
    assert_eq!(produced, committed, "change-structural-system/switches-structural-system-to-wall-dcm: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `structuralSystem` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-structural-system`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn change_structural_system_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-structural-system committed diff decodes");
    assert_eq!(decoded.structural_system.as_deref(), Some("wall_dcm"), "change-structural-system/switches-structural-system-to-wall-dcm: the committed diff must carry structural_system at wall_dcm");
    assert!(decoded.selected_check_index.is_none(), "change-structural-system/switches-structural-system-to-wall-dcm: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-structural-system committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-structural-system committed diff reparses");
    assert_eq!(reencoded, original, "change-structural-system/switches-structural-system-to-wall-dcm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the moment_frame_dch to wall_dcm delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn change_structural_system_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-structural-system committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-structural-system committed diff applies to the before-snapshot");
    assert_eq!(produced.structural_system, "wall_dcm", "change-structural-system/switches-structural-system-to-wall-dcm: the committed diff must leave structural_system reading wall_dcm");
    assert_eq!(produced, expected_after(), "change-structural-system/switches-structural-system-to-wall-dcm: the committed diff did not carry before to after");
}
