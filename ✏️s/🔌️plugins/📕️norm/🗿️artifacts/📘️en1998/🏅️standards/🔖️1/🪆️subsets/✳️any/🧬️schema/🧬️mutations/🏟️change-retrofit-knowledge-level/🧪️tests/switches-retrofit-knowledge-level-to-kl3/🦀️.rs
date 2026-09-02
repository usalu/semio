//! 🧪️ `change-retrofit-knowledge-level` fixture — `switches-retrofit-knowledge-level-to-kl3`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️.rs`, which writes `En1998Diff.retrofit_knowledge_level` and nothing else,
//! behind a `base.retrofit_knowledge_level == payload.new_retrofit_knowledge_level` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard).
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
    serde_json::from_str(BEFORE).expect("change-retrofit-knowledge-level before-snapshot decodes")
}
fn expected_after() -> En1998Snapshot {
    serde_json::from_str(AFTER).expect("change-retrofit-knowledge-level after-snapshot decodes")
}
fn mutation() -> En1998Mutation {
    serde_json::from_str(MUTATION).expect("change-retrofit-knowledge-level mutation decodes")
}

/// ▶️ `change-retrofit-knowledge-level` carries the committed before-snapshot to the committed after-snapshot by moving
/// `retrofit_knowledge_level` from kl2 to kl3, leaving every other EN 1998 seismic-design input alone.
#[semio_framework_async_macros::async_test]
fn change_retrofit_knowledge_level_applies_to_committed_after() {
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("change-retrofit-knowledge-level applies to its committed before-snapshot");
    assert_eq!(applied.retrofit_knowledge_level, "kl3", "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: retrofit_knowledge_level must read kl3 after the change");
    assert_eq!(applied, expected_after(), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: a real kl2 to kl3 change must raise no `mutation.no-op` message");
}

/// ↩️ `change-retrofit-knowledge-level` is its own inverse partner: the inverse step restores `retrofit_knowledge_level` to its pre-change
/// kl2 and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
fn change_retrofit_knowledge_level_inverse_restores_before() {
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward change-retrofit-knowledge-level applies");
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse change-retrofit-knowledge-level step applies");
        restored = next;
    }
    assert_eq!(restored.retrofit_knowledge_level, "kl2", "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: the inverse must put retrofit_knowledge_level back to kl2");
    assert_eq!(restored, base, "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: the inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `ChangeRetrofitKnowledgeLevel` payload are already canonical:
/// decode then encode is a fixed point, so `retrofitKnowledgeLevel` and `newRetrofitKnowledgeLevel` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
fn change_retrofit_knowledge_level_committed_json_is_canonical() {
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {
        let decoded: En1998Snapshot = serde_json::from_str(text).expect("change-retrofit-knowledge-level snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-retrofit-knowledge-level snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-retrofit-knowledge-level snapshot reparses");
        assert_eq!(reencoded, original, "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-retrofit-knowledge-level mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-retrofit-knowledge-level mutation reparses");
    assert_eq!(reencoded, original, "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: committed mutation JSON is not the canonical externally-tagged ChangeRetrofitKnowledgeLevel form carrying newRetrofitKnowledgeLevel");
}

/// 🎯️ The declared outcome holds: `change-retrofit-knowledge-level` at kl3 is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
fn change_retrofit_knowledge_level_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-retrofit-knowledge-level outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: this fixture declares an applied outcome");
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: moving retrofit_knowledge_level from kl2 to kl3 must raise no `mutation.no-op` message");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: an applied outcome must survive the diff-apply seam");
}

/// 🔺️ The sparse delta `change-retrofit-knowledge-level` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `retrofitKnowledgeLevel` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `retrofitLimitState`.
#[semio_framework_async_macros::async_test]
fn change_retrofit_knowledge_level_produces_committed_diff() {
    let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff().retrofit_knowledge_level.as_deref(), Some("kl3"), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: the diff must set retrofit_knowledge_level to kl3");
    assert!(outcome.diff().artifact.is_none(), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().retrofit_limit_state.is_none(), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: change-retrofit-knowledge-level must leave retrofit_limit_state untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("change-retrofit-knowledge-level produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-retrofit-knowledge-level committed diff decodes");
    assert_eq!(produced, committed, "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes back into `En1998Diff` with `retrofitKnowledgeLevel` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `change-retrofit-knowledge-level`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
fn change_retrofit_knowledge_level_committed_diff_is_canonical() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-retrofit-knowledge-level committed diff decodes");
    assert_eq!(decoded.retrofit_knowledge_level.as_deref(), Some("kl3"), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: the committed diff must carry retrofit_knowledge_level at kl3");
    assert!(decoded.selected_check_index.is_none(), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("change-retrofit-knowledge-level committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-retrofit-knowledge-level committed diff reparses");
    assert_eq!(reencoded, original, "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the kl2 to kl3 delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn change_retrofit_knowledge_level_committed_diff_applies_to_after() {
    let decoded: En1998Diff = serde_json::from_str(DIFF).expect("change-retrofit-knowledge-level committed diff decodes");
    let produced = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&decoded, &before()).expect("change-retrofit-knowledge-level committed diff applies to the before-snapshot");
    assert_eq!(produced.retrofit_knowledge_level, "kl3", "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: the committed diff must leave retrofit_knowledge_level reading kl3");
    assert_eq!(produced, expected_after(), "change-retrofit-knowledge-level/switches-retrofit-knowledge-level-to-kl3: the committed diff did not carry before to after");
}
