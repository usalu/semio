//! 🧪️ `replace-part-number-rule` fixture — `swaps-the-literal-rule-for-a-height-driven-script`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `replace-part-number-rule` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
//! the nested states `None` and `Some(None)` are NOT distinguishable in this file's committed diff,
//! and nothing here asserts that they are.

use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Iso16757Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> Iso16757Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> Iso16757Mutation {
    serde_json::from_str(MUTATION).expect("the committed `replace-part-number-rule` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `replace-part-number-rule` swaps the whole tagged `PartNumberRule` enum, moving it from the `Literal`
/// variant to the `Script` variant. The script's `source` names `height`, which the committed
/// `part_number_inputs` supplies — but the oracle does not resolve inputs, so nothing about that binding is
/// checked here.
#[semio_framework_async_macros::async_test]
async fn swaps_the_literal_rule_for_a_height_driven_script() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("replace-part-number-rule applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the applied state differs from the committed after-snapshot");
    assert!(matches!(applied.part_number_rule, crate::artifacts::iso16757::part_5::PartNumberRule::Script { .. }), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the rule must land on the Script variant");
    assert_eq!(applied.part_number_inputs, before().part_number_inputs, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: replacing the rule must not touch the inputs it will read");
    assert_eq!(applied.script_limits, before().script_limits, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: nor the budgets the script will run under");
}

/// ↩️ `replace-part-number-rule`'s inverse clones the OLD rule out of BASE, so replaying it puts the `Literal {
/// value: "PR-600" }` variant back.
#[semio_framework_async_macros::async_test]
async fn restoring_the_literal_rule_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward replace-part-number-rule applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the inverse of one rule replacement is exactly one replacement back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the replace-part-number-rule inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: restoring the literal rule did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `replace-part-number-rule` payload are already canonical:
/// decode → encode is a fixed point. The committed payload is spelled `{"ReplacePartNumberRule": {"new_rule":
/// {"kind": "script", "function_id": …}}}` — `PartNumberRule` is internally tagged on `kind` with camelCase
/// VARIANTS, but its struct-variant FIELDS keep snake_case.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the replace-part-number-rule payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the replace-part-number-rule payload reparses");
    assert_eq!(reencoded, original, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the committed replace-part-number-rule JSON is not canonical");
}

/// 🎯️ The `Script` rule is not equal to the committed `Literal` rule, so the whole-enum equality guard does not
/// raise `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the Script variant differs from the committed Literal variant, so `replace-part-number-rule`'s `mutation.no-op` guard cannot fire"
    );
    assert!(produced.messages().is_empty(), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: an accepted replace-part-number-rule emits no diagnostics at all");
}

/// 🔺️ The sparse delta `replace-part-number-rule` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only
/// `partNumberRule` is rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced replace-part-number-rule diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the replacement part-
/// number rule and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed replace-part-number-rule diff decodes");
    let rule = decoded.part_number_rule.as_ref().expect("the committed replace-part-number-rule diff carries the rule");
    assert!(matches!(rule, crate::artifacts::iso16757::part_5::PartNumberRule::Script { .. }), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the diff must carry the Script variant");
    assert!(decoded.catalogue.is_none(), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: replace-part-number-rule writes `partNumberRule` and must leave `catalogue` untouched");
    assert!(decoded.dictionary.is_none(), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: replace-part-number-rule writes `partNumberRule` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: replace-part-number-rule writes `partNumberRule` and must leave `selection` untouched");
    assert!(decoded.part_number_inputs.is_none(), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: replace-part-number-rule writes `partNumberRule` and must leave `part_number_inputs` untouched");
    assert!(decoded.artifact.is_none(), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the part-number rule replacement, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed replace-part-number-rule diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-part-number-rule/swaps-the-literal-rule-for-a-height-driven-script: the committed diff did not carry before to after");
}
