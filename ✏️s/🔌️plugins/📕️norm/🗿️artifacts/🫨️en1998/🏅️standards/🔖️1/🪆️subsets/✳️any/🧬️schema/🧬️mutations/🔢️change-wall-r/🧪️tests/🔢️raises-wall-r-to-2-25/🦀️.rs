//! 🧪️ `change-wall-r` fixture — `🔢️raises-wall-r-to-2-25`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.wall_r` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.wall_r == payload.new_wall_r` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-wall-r before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-wall-r after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-wall-r mutation decodes")
}

/// ▶️ `change-wall-r` carries the committed before-snapshot to the committed after-snapshot by moving
/// `wall_r` from 1.5 to 2.25, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_wall_r_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-wall-r applies to its committed before-snapshot");
    assert_eq!(applied.wall_r, 2.25, "change-wall-r/raises-wall-r-to-2-25: wall_r must read 2.25 after the change");
    assert_eq!(applied, expected_after(), "change-wall-r/raises-wall-r-to-2-25: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-wall-r/raises-wall-r-to-2-25: a real 1.5 to 2.25 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-wall-r` is its own inverse partner: the inverse step restores `wall_r` to its pre-change
/// 1.5 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_wall_r_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-wall-r applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-wall-r/raises-wall-r-to-2-25: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-wall-r step applies");
        restored = next;
    }
    assert_eq!(restored.wall_r, 1.5, "change-wall-r/raises-wall-r-to-2-25: the inverse must put wall_r back to 1.5");
    assert_eq!(restored, base, "change-wall-r/raises-wall-r-to-2-25: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeWallR` payload are already canonical:
/// decode then encode is a fixed point, so `wallR` and `newWallR` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_wall_r_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-wall-r snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-wall-r snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-wall-r snapshot reparses");
        assert_eq!(reencoded, original, "change-wall-r/raises-wall-r-to-2-25: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-wall-r mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-wall-r mutation reparses");
    assert_eq!(reencoded, original, "change-wall-r/raises-wall-r-to-2-25: committed mutation JSON is not the canonical externally-tagged ChangeWallR form carrying newWallR");
}

/// 🎯️ The declared outcome holds: `change-wall-r` at 2.25 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_wall_r_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-wall-r outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-wall-r/raises-wall-r-to-2-25: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-wall-r/raises-wall-r-to-2-25: moving wall_r from 1.5 to 2.25 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-wall-r/raises-wall-r-to-2-25: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-wall-r` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `wallR` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `wallHRdKn`.
#[semio_framework_async_macros::async_test]
fn change_wall_r_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().wall_r, Some(2.25), "change-wall-r/raises-wall-r-to-2-25: the diff must set wall_r to 2.25");
    assert!(outcome.diff().artifact.is_none(), "change-wall-r/raises-wall-r-to-2-25: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().wall_h_rd_kn.is_none(), "change-wall-r/raises-wall-r-to-2-25: change-wall-r must leave wall_h_rd_kn untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-wall-r produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-wall-r committed diff decodes");
    assert_eq!(produced, committed, "change-wall-r/raises-wall-r-to-2-25: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `wallR` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-wall-r`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_wall_r_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-wall-r committed diff decodes");
    assert_eq!(decoded.wall_r, Some(2.25), "change-wall-r/raises-wall-r-to-2-25: the committed diff must carry wall_r at 2.25");
    assert!(decoded.selected_check_index.is_none(), "change-wall-r/raises-wall-r-to-2-25: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-wall-r committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-wall-r committed diff reparses");
    assert_eq!(reencoded, original, "change-wall-r/raises-wall-r-to-2-25: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 1.5 to 2.25 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_wall_r_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-wall-r committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-wall-r committed diff applies to the before-snapshot");
    assert_eq!(produced.wall_r, 2.25, "change-wall-r/raises-wall-r-to-2-25: the committed diff must leave wall_r reading 2.25");
    assert_eq!(produced, expected_after(), "change-wall-r/raises-wall-r-to-2-25: the committed diff did not carry before to after");
}
