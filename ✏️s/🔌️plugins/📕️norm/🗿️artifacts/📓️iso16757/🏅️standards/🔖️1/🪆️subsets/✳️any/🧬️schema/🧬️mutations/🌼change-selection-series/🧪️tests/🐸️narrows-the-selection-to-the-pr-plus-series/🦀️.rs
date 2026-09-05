//! 🧪️ `change-selection-series` fixture — `🐸️narrows-the-selection-to-the-pr-plus-series`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-selection-series` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `change-selection-series` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `change-selection-series` writes `selection.series_id`, whose payload type is `Option<String>` — this case
/// carries `Some`, so it swaps one series id for another rather than clearing the field.
#[semio_framework_async_macros::async_test]
fn narrows_the_selection_to_the_pr_plus_series() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-selection-series applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.selection.series_id.as_deref(), Some("series.pr-plus"), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the selection series must be narrowed");
    assert_eq!(applied.selection.class_id, before().selection.class_id, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the class id is a sibling field of the same request and must not move");
    assert_eq!(applied.selection.constraints.len(), 2, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: narrowing by series must not drop property constraints");
}

/// ↩️ `change-selection-series`'s inverse reads the OLD `Option<String>` out of BASE and replays it wholesale,
/// so a `Some("series.pr")` goes back exactly as it was — the same code path that would restore a `None`.
#[semio_framework_async_macros::async_test]
fn widening_back_to_the_pr_series_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-selection-series applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the inverse of one selection-series change is exactly one change back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-selection-series inverse step applies");
    }
    assert_eq!(snapshot, base, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: widening back to `series.pr` did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-selection-series` payload are already canonical: decode
/// → encode is a fixed point. The committed payload is spelled `{"ChangeSelectionSeries": {"new_series_id":
/// "series.pr-plus"}}` — the payload field is an `Option<String>` with no `skip_serializing_if`, so a cleared
/// series would encode as an explicit `null` here.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-selection-series payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-selection-series payload reparses");
    assert_eq!(reencoded, original, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the committed change-selection-series JSON is not canonical");
}

/// 🎯️ `Some("series.pr-plus")` differs from the committed `Some("series.pr")`, so the `Option`-level equality
/// guard stays shut.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the guard compares whole `Option<String>` values and the two differ, so `change-selection-series`'s `mutation.no-op` warning cannot fire"
    );
    assert!(produced.messages().is_empty(), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: an accepted change-selection-series emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-selection-series` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `selection` is
/// rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-selection-series diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// selection request and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed change-selection-series diff decodes");
    let selection = decoded.selection.as_ref().expect("the committed change-selection-series diff carries the selection request");
    assert_eq!(selection.series_id.as_deref(), Some("series.pr-plus"), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the diff must carry the new series id");
    assert_eq!(selection.class_id, "class.panel-radiator", "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the class id rides through the whole-container delta unchanged");
    assert!(decoded.catalogue.is_none(), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: change-selection-series writes `selection` and must leave `catalogue` untouched");
    assert!(decoded.dictionary.is_none(), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: change-selection-series writes `selection` and must leave `dictionary` untouched");
    assert!(decoded.part_number_rule.is_none(), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: change-selection-series writes `selection` and must leave `part_number_rule` untouched");
    assert!(decoded.script_limits.is_none(), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: change-selection-series writes `selection` and must leave `script_limits` untouched");
    assert!(decoded.artifact.is_none(), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the selection-series change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed change-selection-series diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-selection-series/narrows-the-selection-to-the-pr-plus-series: the committed diff did not carry before to after");
}
