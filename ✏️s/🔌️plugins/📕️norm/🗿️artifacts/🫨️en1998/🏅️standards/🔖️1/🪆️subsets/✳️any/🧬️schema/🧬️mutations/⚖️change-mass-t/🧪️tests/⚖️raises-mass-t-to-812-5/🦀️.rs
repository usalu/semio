//! 🧪️ `change-mass-t` fixture — `⚖️raises-mass-t-to-812-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.mass_t` and nothing else,
//! behind an `is_finite` `mutation.invariant` guard and a `base.mass_t == payload.new_mass_t` `mutation.no-op` guard.
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
    serde_json::from_str(BEFORE).expect("change-mass-t before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-mass-t after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-mass-t mutation decodes")
}

/// ▶️ `change-mass-t` carries the committed before-snapshot to the committed after-snapshot by moving
/// `mass_t` from 500.0 to 812.5, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_mass_t_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-mass-t applies to its committed before-snapshot");
    assert_eq!(applied.mass_t, 812.5, "change-mass-t/raises-mass-t-to-812-5: mass_t must read 812.5 after the change");
    assert_eq!(applied, expected_after(), "change-mass-t/raises-mass-t-to-812-5: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-mass-t/raises-mass-t-to-812-5: a real 500.0 to 812.5 change must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
}

/// ↩️ `change-mass-t` is its own inverse partner: the inverse step restores `mass_t` to its pre-change
/// 500.0 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_mass_t_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-mass-t applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-mass-t/raises-mass-t-to-812-5: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-mass-t step applies");
        restored = next;
    }
    assert_eq!(restored.mass_t, 500.0, "change-mass-t/raises-mass-t-to-812-5: the inverse must put mass_t back to 500.0");
    assert_eq!(restored, base, "change-mass-t/raises-mass-t-to-812-5: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeMassT` payload are already canonical:
/// decode then encode is a fixed point, so `massT` and `newMassT` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_mass_t_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-mass-t snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-mass-t snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-mass-t snapshot reparses");
        assert_eq!(reencoded, original, "change-mass-t/raises-mass-t-to-812-5: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-mass-t mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-mass-t mutation reparses");
    assert_eq!(reencoded, original, "change-mass-t/raises-mass-t-to-812-5: committed mutation JSON is not the canonical externally-tagged ChangeMassT form carrying newMassT");
}

/// 🎯️ The declared outcome holds: `change-mass-t` at 812.5 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_mass_t_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-mass-t outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-mass-t/raises-mass-t-to-812-5: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-mass-t/raises-mass-t-to-812-5: moving mass_t from 500.0 to 812.5 must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-mass-t/raises-mass-t-to-812-5: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-mass-t` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `massT` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `vRdKn`.
#[semio_framework_async_macros::async_test]
fn change_mass_t_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().mass_t, Some(812.5), "change-mass-t/raises-mass-t-to-812-5: the diff must set mass_t to 812.5");
    assert!(outcome.diff().artifact.is_none(), "change-mass-t/raises-mass-t-to-812-5: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().v_rd_kn.is_none(), "change-mass-t/raises-mass-t-to-812-5: change-mass-t must leave v_rd_kn untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-mass-t produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-mass-t committed diff decodes");
    assert_eq!(produced, committed, "change-mass-t/raises-mass-t-to-812-5: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `massT` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-mass-t`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_mass_t_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-mass-t committed diff decodes");
    assert_eq!(decoded.mass_t, Some(812.5), "change-mass-t/raises-mass-t-to-812-5: the committed diff must carry mass_t at 812.5");
    assert!(decoded.selected_check_index.is_none(), "change-mass-t/raises-mass-t-to-812-5: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-mass-t committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-mass-t committed diff reparses");
    assert_eq!(reencoded, original, "change-mass-t/raises-mass-t-to-812-5: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the 500.0 to 812.5 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_mass_t_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-mass-t committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-mass-t committed diff applies to the before-snapshot");
    assert_eq!(produced.mass_t, 812.5, "change-mass-t/raises-mass-t-to-812-5: the committed diff must leave mass_t reading 812.5");
    assert_eq!(produced, expected_after(), "change-mass-t/raises-mass-t-to-812-5: the committed diff did not carry before to after");
}
