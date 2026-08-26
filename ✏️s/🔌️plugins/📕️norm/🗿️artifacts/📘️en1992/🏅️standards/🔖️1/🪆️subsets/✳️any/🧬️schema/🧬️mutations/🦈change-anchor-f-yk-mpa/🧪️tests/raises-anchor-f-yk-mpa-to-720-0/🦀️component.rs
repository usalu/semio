//! 🧪️ `change-anchor-f-yk-mpa` fixture — `raises-anchor-f-yk-mpa-to-720-0`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `En1992Diff.anchor_f_yk_mpa` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.anchor_f_yk_mpa == payload.new_anchor_f_yk_mpa` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-anchor-f-yk-mpa before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-anchor-f-yk-mpa after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-anchor-f-yk-mpa mutation decodes")
}

/// ▶️ `change-anchor-f-yk-mpa` carries the committed before-snapshot to the committed after-snapshot by moving
/// `anchor_f_yk_mpa` from 640.0 to 720.0, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_yk_mpa_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-anchor-f-yk-mpa applies to its committed before-snapshot");
    assert_eq!(applied.anchor_f_yk_mpa, 720.0, "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: anchor_f_yk_mpa must read 720.0 after the change");
    assert_eq!(applied, expected_after(), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: a real 640.0 to 720.0 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-anchor-f-yk-mpa` is its own inverse partner: the inverse step restores `anchor_f_yk_mpa` to its pre-change
/// 640.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_yk_mpa_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-anchor-f-yk-mpa applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-anchor-f-yk-mpa step applies");
        restored = next;
    }
    assert_eq!(restored.anchor_f_yk_mpa, 640.0, "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: the inverse must put anchor_f_yk_mpa back to 640.0");
    assert_eq!(restored, base, "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeAnchorFYkMpa` payload are already canonical:
/// decode then encode is a fixed point, so `anchorFYkMpa` and `newAnchorFYkMpa` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_yk_mpa_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-anchor-f-yk-mpa snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-anchor-f-yk-mpa snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-anchor-f-yk-mpa snapshot reparses");
        assert_eq!(reencoded, original, "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-anchor-f-yk-mpa mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-anchor-f-yk-mpa mutation reparses");
    assert_eq!(reencoded, original, "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: committed mutation JSON is not the canonical externally-tagged ChangeAnchorFYkMpa form carrying newAnchorFYkMpa");
}

/// 🎯️ The declared outcome holds: `change-anchor-f-yk-mpa` at 720.0 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_yk_mpa_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-anchor-f-yk-mpa outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: moving anchor_f_yk_mpa from 640.0 to 720.0 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-anchor-f-yk-mpa` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `anchorFYkMpa` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `anchorASMm2`.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_yk_mpa_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().anchor_f_yk_mpa, Some(720.0), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: the diff must set anchor_f_yk_mpa to 720.0");
    assert!(outcome.diff().artifact.is_none(), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().anchor_a_s_mm2.is_none(), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: change-anchor-f-yk-mpa must leave anchor_a_s_mm2 untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-anchor-f-yk-mpa produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-anchor-f-yk-mpa committed diff decodes");
    assert_eq!(produced, committed, "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `anchorFYkMpa` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-anchor-f-yk-mpa`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_yk_mpa_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-anchor-f-yk-mpa committed diff decodes");
    assert_eq!(decoded.anchor_f_yk_mpa, Some(720.0), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: the committed diff must carry anchor_f_yk_mpa at 720.0");
    assert!(decoded.selected_check_index.is_none(), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-anchor-f-yk-mpa committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-anchor-f-yk-mpa committed diff reparses");
    assert_eq!(reencoded, original, "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 640.0 to 720.0 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_anchor_f_yk_mpa_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-anchor-f-yk-mpa committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-anchor-f-yk-mpa committed diff applies to the before-snapshot");
    assert_eq!(produced.anchor_f_yk_mpa, 720.0, "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: the committed diff must leave anchor_f_yk_mpa reading 720.0");
    assert_eq!(produced, expected_after(), "change-anchor-f-yk-mpa/raises-anchor-f-yk-mpa-to-720-0: the committed diff did not carry before to after");
}
