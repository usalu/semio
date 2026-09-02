//! 🧪️ `change-k-soil` fixture — `raises-k-soil-to-262500-0`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.k_soil` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.k_soil == payload.new_k_soil` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-k-soil before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-k-soil after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-k-soil mutation decodes")
}

/// ▶️ `change-k-soil` carries the committed before-snapshot to the committed after-snapshot by moving
/// `k_soil` from 200000.0 to 262500.0, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_k_soil_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-k-soil applies to its committed before-snapshot");
    assert_eq!(applied.k_soil, 262500.0, "change-k-soil/raises-k-soil-to-262500-0: k_soil must read 262500.0 after the change");
    assert_eq!(applied, expected_after(), "change-k-soil/raises-k-soil-to-262500-0: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-k-soil/raises-k-soil-to-262500-0: a real 200000.0 to 262500.0 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-k-soil` is its own inverse partner: the inverse step restores `k_soil` to its pre-change
/// 200000.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_k_soil_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-k-soil applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-k-soil/raises-k-soil-to-262500-0: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-k-soil step applies");
        restored = next;
    }
    assert_eq!(restored.k_soil, 200000.0, "change-k-soil/raises-k-soil-to-262500-0: the inverse must put k_soil back to 200000.0");
    assert_eq!(restored, base, "change-k-soil/raises-k-soil-to-262500-0: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeKSoil` payload are already canonical:
/// decode then encode is a fixed point, so `kSoil` and `newKSoil` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_k_soil_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-k-soil snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-k-soil snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-k-soil snapshot reparses");
        assert_eq!(reencoded, original, "change-k-soil/raises-k-soil-to-262500-0: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-k-soil mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-k-soil mutation reparses");
    assert_eq!(reencoded, original, "change-k-soil/raises-k-soil-to-262500-0: committed mutation JSON is not the canonical externally-tagged ChangeKSoil form carrying newKSoil");
}

/// 🎯️ The declared outcome holds: `change-k-soil` at 262500.0 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_k_soil_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-k-soil outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-k-soil/raises-k-soil-to-262500-0: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-k-soil/raises-k-soil-to-262500-0: moving k_soil from 200000.0 to 262500.0 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-k-soil/raises-k-soil-to-262500-0: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-k-soil` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `kSoil` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `wallHeightM`.
#[semio_framework_async_macros::async_test]
fn change_k_soil_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().k_soil, Some(262500.0), "change-k-soil/raises-k-soil-to-262500-0: the diff must set k_soil to 262500.0");
    assert!(outcome.diff().artifact.is_none(), "change-k-soil/raises-k-soil-to-262500-0: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().wall_height_m.is_none(), "change-k-soil/raises-k-soil-to-262500-0: change-k-soil must leave wall_height_m untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-k-soil produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-k-soil committed diff decodes");
    assert_eq!(produced, committed, "change-k-soil/raises-k-soil-to-262500-0: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `kSoil` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-k-soil`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_k_soil_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-k-soil committed diff decodes");
    assert_eq!(decoded.k_soil, Some(262500.0), "change-k-soil/raises-k-soil-to-262500-0: the committed diff must carry k_soil at 262500.0");
    assert!(decoded.selected_check_index.is_none(), "change-k-soil/raises-k-soil-to-262500-0: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-k-soil committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-k-soil committed diff reparses");
    assert_eq!(reencoded, original, "change-k-soil/raises-k-soil-to-262500-0: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 200000.0 to 262500.0 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_k_soil_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-k-soil committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-k-soil committed diff applies to the before-snapshot");
    assert_eq!(produced.k_soil, 262500.0, "change-k-soil/raises-k-soil-to-262500-0: the committed diff must leave k_soil reading 262500.0");
    assert_eq!(produced, expected_after(), "change-k-soil/raises-k-soil-to-262500-0: the committed diff did not carry before to after");
}
