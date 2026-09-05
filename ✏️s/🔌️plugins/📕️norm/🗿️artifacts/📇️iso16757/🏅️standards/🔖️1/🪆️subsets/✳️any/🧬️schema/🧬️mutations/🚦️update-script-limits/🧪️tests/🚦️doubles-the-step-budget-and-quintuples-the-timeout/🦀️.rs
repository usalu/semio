//! 🧪️ `update-script-limits` fixture — `🚦️doubles-the-step-budget-and-quintuples-the-timeout`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `update-script-limits` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
//! the nested states `None` and `Some(None)` are NOT distinguishable in this file's committed diff,
//! and nothing here asserts that they are.

use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Iso16757Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> Iso16757Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> Iso16757Mutation {
    serde_json::from_str(MUTATION).expect("the committed `update-script-limits` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `update-script-limits` is the tree's one `update-<facet>` verb: it rebuilds the whole `ScriptLimits`
/// triple from three payload fields at once, because the three budgets are validated as one atomic bundle
/// rather than as independent rows.
#[semio_framework_async_macros::async_test]
fn doubles_the_step_budget_and_quintuples_the_timeout() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("update-script-limits applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.script_limits.max_steps, 20000, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the step budget must double");
    assert_eq!(applied.script_limits.max_recursion, 128, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the recursion budget must double");
    assert_eq!(applied.script_limits.timeout_ms, 250, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the timeout must reach 250 ms");
}

/// ↩️ `update-script-limits`'s inverse reads all three OLD budgets out of BASE into one `UpdateScriptLimits`, so
/// the atomic bundle is restored atomically.
#[semio_framework_async_macros::async_test]
fn restoring_the_default_budgets_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward update-script-limits applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the inverse of one script-limits update is exactly one update back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the update-script-limits inverse step applies");
    }
    assert_eq!(snapshot, base, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: restoring the 10000/64/50 budgets did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `update-script-limits` payload are already canonical: decode →
/// encode is a fixed point. The committed payload is spelled `{"UpdateScriptLimits": {"new_max_steps": …,
/// "new_max_recursion": …, "new_timeout_ms": …}}` — three bare JSON integers (`u32`, `u32`, `u64`),
/// snake_case.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the update-script-limits payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the update-script-limits payload reparses");
    assert_eq!(reencoded, original, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the committed update-script-limits JSON is not canonical");
}

/// 🎯️ The oracle builds the candidate `ScriptLimits` first and compares the WHOLE struct; 20000/128/250 differs
/// from the committed 10000/64/50, so `mutation.no-op` stays shut.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the rebuilt ScriptLimits differs from the committed 10000/64/50 triple, so the whole-struct `mutation.no-op` guard cannot fire");
    assert!(produced.messages().is_empty(), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: an accepted update-script-limits emits no diagnostics at all");
}

/// 🔺️ The sparse delta `update-script-limits` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `scriptLimits`
/// is rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced update-script-limits diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the rebuilt script-limits
/// triple and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed update-script-limits diff decodes");
    let limits = decoded.script_limits.as_ref().expect("the committed update-script-limits diff carries the limits");
    assert_eq!((limits.max_steps, limits.max_recursion, limits.timeout_ms), (20000, 128, 250), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the diff must carry all three new budgets together");
    assert!(decoded.catalogue.is_none(), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: update-script-limits writes `scriptLimits` and must leave `catalogue` untouched");
    assert!(decoded.dictionary.is_none(), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: update-script-limits writes `scriptLimits` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: update-script-limits writes `scriptLimits` and must leave `selection` untouched");
    assert!(decoded.part_number_rule.is_none(), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: update-script-limits writes `scriptLimits` and must leave `part_number_rule` untouched");
    assert!(decoded.artifact.is_none(), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the script-limits update, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed update-script-limits diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-script-limits/doubles-the-step-budget-and-quintuples-the-timeout: the committed diff did not carry before to after");
}
