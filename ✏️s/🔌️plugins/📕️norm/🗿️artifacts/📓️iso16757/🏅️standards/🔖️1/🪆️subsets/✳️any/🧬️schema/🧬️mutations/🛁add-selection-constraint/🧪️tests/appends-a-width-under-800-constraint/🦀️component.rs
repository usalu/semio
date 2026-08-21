//! 🧪️ `add-selection-constraint` fixture — `appends-a-width-under-800-constraint`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `add-selection-constraint` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `add-selection-constraint` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `add-selection-constraint` PUSHES onto `selection.constraints`, so the new width constraint lands at index
/// 2, after the committed height and length constraints, and the existing two keep their positions.
#[semio_framework_async_macros::async_test]
async fn appends_a_width_under_800_constraint() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("add-selection-constraint applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "add-selection-constraint/appends-a-width-under-800-constraint: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.selection.constraints.len(), 3, "add-selection-constraint/appends-a-width-under-800-constraint: the constraint list must grow by exactly one");
    assert_eq!(applied.selection.constraints[2].property_id, "prop.width", "add-selection-constraint/appends-a-width-under-800-constraint: the new constraint is APPENDED, so it must land at index 2");
    assert_eq!(applied.selection.constraints[0], before().selection.constraints[0], "add-selection-constraint/appends-a-width-under-800-constraint: the pre-existing height constraint must keep both its value and its position");
}

/// ↩️ `add-selection-constraint`'s inverse is a `RemoveSelectionConstraint` addressed at
/// `base.selection.constraints.len()` — index 2 here — which is exactly where the push landed the new entry.
#[semio_framework_async_macros::async_test]
async fn removing_the_appended_constraint_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward add-selection-constraint applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "add-selection-constraint/appends-a-width-under-800-constraint: an append inverts to exactly one RemoveSelectionConstraint at the pre-append length");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the add-selection-constraint inverse step applies");
    }
    assert_eq!(snapshot, base, "add-selection-constraint/appends-a-width-under-800-constraint: removing the appended constraint did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `add-selection-constraint` payload are already canonical:
/// decode → encode is a fixed point. The committed payload is spelled `{"AddSelectionConstraint":
/// {"constraint": {"property_id": …, "operator": "LessThan", "value": {"kind": "decimal", …}}}}` —
/// `ConstraintOperator` carries no serde rename, so it is the bare Rust variant name.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "add-selection-constraint/appends-a-width-under-800-constraint: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the add-selection-constraint payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the add-selection-constraint payload reparses");
    assert_eq!(reencoded, original, "add-selection-constraint/appends-a-width-under-800-constraint: the committed add-selection-constraint JSON is not canonical");
}

/// 🎯️ The committed constraints are on `prop.height` and `prop.length`; a `prop.width` constraint is not
/// `contains`-equal to either, so the `mutation.no-op` duplicate guard stays shut.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "add-selection-constraint/appends-a-width-under-800-constraint: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "add-selection-constraint/appends-a-width-under-800-constraint: the new constraint is not already in the committed list, so `add-selection-constraint`'s `mutation.no-op` guard cannot fire");
    assert!(produced.messages().is_empty(), "add-selection-constraint/appends-a-width-under-800-constraint: an accepted add-selection-constraint emits no diagnostics at all");
}

/// 🔺️ The sparse delta `add-selection-constraint` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `selection` is
/// rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced add-selection-constraint diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "add-selection-constraint/appends-a-width-under-800-constraint: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// selection request and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed add-selection-constraint diff decodes");
    let selection = decoded.selection.as_ref().expect("the committed add-selection-constraint diff carries the selection request");
    assert_eq!(selection.constraints.len(), 3, "add-selection-constraint/appends-a-width-under-800-constraint: the diff carries the whole three-entry list, not just the appended entry");
    assert_eq!(selection.constraints[2].property_id, "prop.width", "add-selection-constraint/appends-a-width-under-800-constraint: the appended constraint must be last in the diff too");
    assert!(decoded.catalogue.is_none(), "add-selection-constraint/appends-a-width-under-800-constraint: add-selection-constraint writes `selection` and must leave `catalogue` untouched");
    assert!(decoded.dictionary.is_none(), "add-selection-constraint/appends-a-width-under-800-constraint: add-selection-constraint writes `selection` and must leave `dictionary` untouched");
    assert!(decoded.part_number_inputs.is_none(), "add-selection-constraint/appends-a-width-under-800-constraint: add-selection-constraint writes `selection` and must leave `part_number_inputs` untouched");
    assert!(decoded.exchange_process.is_none(), "add-selection-constraint/appends-a-width-under-800-constraint: add-selection-constraint writes `selection` and must leave `exchange_process` untouched");
    assert!(decoded.artifact.is_none(), "add-selection-constraint/appends-a-width-under-800-constraint: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "add-selection-constraint/appends-a-width-under-800-constraint: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the constraint append, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed add-selection-constraint diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-selection-constraint/appends-a-width-under-800-constraint: the committed diff did not carry before to after");
}
