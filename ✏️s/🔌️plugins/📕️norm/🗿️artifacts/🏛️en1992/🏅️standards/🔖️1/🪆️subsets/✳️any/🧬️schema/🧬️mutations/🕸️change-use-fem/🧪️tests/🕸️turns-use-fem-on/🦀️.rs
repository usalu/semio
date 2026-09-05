//! 🧪️ `change-use-fem` fixture — `🕸️turns-use-fem-on`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1992Diff.use_fem` and nothing else,
//! behind a `base.use_fem == payload.new_use_fem` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1992Snapshot {
    serde_json::from_str(BEFORE).expect("change-use-fem before-snapshot decodes")
}
fn expected_after() -> En1992Snapshot {
    serde_json::from_str(AFTER).expect("change-use-fem after-snapshot decodes")
}
fn mutation() -> En1992Mutation {
    serde_json::from_str(MUTATION).expect("change-use-fem mutation decodes")
}

/// ▶️ `change-use-fem` carries the committed before-snapshot to the committed after-snapshot by moving
/// `use_fem` from false to true, leaving every other EN 1992 concrete-design input alone.
#[semio_framework_async_macros::async_test]
fn change_use_fem_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-use-fem applies to its committed before-snapshot");
    assert!(applied.use_fem, "change-use-fem/turns-use-fem-on: use_fem must read true after the change");
    assert_eq!(applied, expected_after(), "change-use-fem/turns-use-fem-on: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-use-fem/turns-use-fem-on: a real false to true change must raise no `mutation.no-op` message");
}

/// ↩️ `change-use-fem` is its own inverse partner: the inverse step restores `use_fem` to its pre-change
/// false and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_use_fem_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-use-fem applies");
    let inverse = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-use-fem/turns-use-fem-on: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-use-fem step applies");
        restored = next;
    }
    assert!(!restored.use_fem, "change-use-fem/turns-use-fem-on: the inverse must put use_fem back to false");
    assert_eq!(restored, base, "change-use-fem/turns-use-fem-on: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeUseFem` payload are already canonical:
/// decode then encode is a fixed point, so `useFem` and `newUseFem` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_use_fem_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1992Snapshot = serde_json::from_str(text).expect("change-use-fem snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-use-fem snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-use-fem snapshot reparses");
        assert_eq!(reencoded, original, "change-use-fem/turns-use-fem-on: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-use-fem mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-use-fem mutation reparses");
    assert_eq!(reencoded, original, "change-use-fem/turns-use-fem-on: committed mutation JSON is not the canonical externally-tagged ChangeUseFem form carrying newUseFem");
}

/// 🎯️ The declared outcome holds: `change-use-fem` at true is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_use_fem_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-use-fem outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-use-fem/turns-use-fem-on: this fixture declares an applied outcome");
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-use-fem/turns-use-fem-on: moving use_fem from false to true must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-use-fem/turns-use-fem-on: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-use-fem` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `useFem` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `spanM`.
#[semio_framework_async_macros::async_test]
fn change_use_fem_produces_committed_diff() {
    let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().use_fem, Some(true), "change-use-fem/turns-use-fem-on: the diff must set use_fem to true");
    assert!(outcome.diff().artifact.is_none(), "change-use-fem/turns-use-fem-on: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().span_m.is_none(), "change-use-fem/turns-use-fem-on: change-use-fem must leave span_m untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-use-fem produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-use-fem committed diff decodes");
    assert_eq!(produced, committed, "change-use-fem/turns-use-fem-on: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1992Diff` with `useFem` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-use-fem`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_use_fem_committed_diff_is_canonical() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-use-fem committed diff decodes");
    assert_eq!(decoded.use_fem, Some(true), "change-use-fem/turns-use-fem-on: the committed diff must carry use_fem at true");
    assert!(decoded.selected_check_index.is_none(), "change-use-fem/turns-use-fem-on: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-use-fem committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-use-fem committed diff reparses");
    assert_eq!(reencoded, original, "change-use-fem/turns-use-fem-on: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the false to true delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_use_fem_committed_diff_applies_to_after() {
    let decoded: En1992Diff = serde_json::from_str(DIFF).expect("change-use-fem committed diff decodes");
    let produced = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&decoded, &before()).expect("change-use-fem committed diff applies to the before-snapshot");
    assert!(produced.use_fem, "change-use-fem/turns-use-fem-on: the committed diff must leave use_fem reading true");
    assert_eq!(produced, expected_after(), "change-use-fem/turns-use-fem-on: the committed diff did not carry before to after");
}
