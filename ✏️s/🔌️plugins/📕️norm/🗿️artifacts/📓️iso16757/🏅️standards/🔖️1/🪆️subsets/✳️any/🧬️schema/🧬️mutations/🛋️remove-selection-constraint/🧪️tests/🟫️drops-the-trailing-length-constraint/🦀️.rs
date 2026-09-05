//! 🧪️ `remove-selection-constraint` fixture — `🟫️drops-the-trailing-length-constraint`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `remove-selection-constraint` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `remove-selection-constraint` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `remove-selection-constraint` is INDEX-addressed, not id-addressed. This case removes index 1 — the
/// trailing length constraint — leaving the height constraint at index 0.
#[semio_framework_async_macros::async_test]
fn drops_the_trailing_length_constraint() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("remove-selection-constraint applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "remove-selection-constraint/drops-the-trailing-length-constraint: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.selection.constraints.len(), 1, "remove-selection-constraint/drops-the-trailing-length-constraint: the constraint list must shrink by exactly one");
    assert_eq!(applied.selection.constraints[0].property_id, "prop.height", "remove-selection-constraint/drops-the-trailing-length-constraint: the surviving constraint is the one that was at index 0");
    assert_eq!(applied.selection.class_id, before().selection.class_id, "remove-selection-constraint/drops-the-trailing-length-constraint: removing a constraint must not retarget the request");
}

/// ↩️ `remove-selection-constraint`'s inverse is an `AddSelectionConstraint`, which PUSHES — so it restores the
/// removed entry only because this case deliberately removes the LAST constraint. Removing an interior index
/// would invert to a different order, and this fixture pins the boundary the inverse is exact at.
#[semio_framework_async_macros::async_test]
fn re_appending_the_length_constraint_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward remove-selection-constraint applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "remove-selection-constraint/drops-the-trailing-length-constraint: removing the trailing constraint inverts to exactly one AddSelectionConstraint that pushes it back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the remove-selection-constraint inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-selection-constraint/drops-the-trailing-length-constraint: re-appending the length constraint did not restore the before-snapshot — the push-based inverse is exact only for the trailing index");
}

/// 🔣️ Both committed snapshots and the committed `remove-selection-constraint` payload are already canonical:
/// decode → encode is a fixed point. The committed payload is spelled `{"RemoveSelectionConstraint":
/// {"index": 1}}` — a bare JSON integer, because the payload field is a `usize`.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "remove-selection-constraint/drops-the-trailing-length-constraint: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the remove-selection-constraint payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the remove-selection-constraint payload reparses");
    assert_eq!(reencoded, original, "remove-selection-constraint/drops-the-trailing-length-constraint: the committed remove-selection-constraint JSON is not canonical");
}

/// 🎯️ Index 1 is inside the committed two-entry list, so the `index >= len` bound check does not take the
/// `mutation.target-missing` Error branch.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-selection-constraint/drops-the-trailing-length-constraint: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "remove-selection-constraint/drops-the-trailing-length-constraint: index 1 is within the committed two-entry constraint list, so `remove-selection-constraint`'s `mutation.target-missing` error cannot fire"
    );
    assert!(produced.messages().is_empty(), "remove-selection-constraint/drops-the-trailing-length-constraint: an accepted remove-selection-constraint emits no diagnostics at all");
}

/// 🔺️ The sparse delta `remove-selection-constraint` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `selection` is
/// rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced remove-selection-constraint diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "remove-selection-constraint/drops-the-trailing-length-constraint: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole surviving
/// selection request and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed remove-selection-constraint diff decodes");
    let selection = decoded.selection.as_ref().expect("the committed remove-selection-constraint diff carries the selection request");
    assert_eq!(selection.constraints.len(), 1, "remove-selection-constraint/drops-the-trailing-length-constraint: a removal is expressed as the SHORTER whole list, never as an index marker");
    assert_eq!(selection.constraints[0].property_id, "prop.height", "remove-selection-constraint/drops-the-trailing-length-constraint: the surviving constraint must be the height one");
    assert!(decoded.catalogue.is_none(), "remove-selection-constraint/drops-the-trailing-length-constraint: remove-selection-constraint writes `selection` and must leave `catalogue` untouched");
    assert!(decoded.dictionary.is_none(), "remove-selection-constraint/drops-the-trailing-length-constraint: remove-selection-constraint writes `selection` and must leave `dictionary` untouched");
    assert!(decoded.part_number_inputs.is_none(), "remove-selection-constraint/drops-the-trailing-length-constraint: remove-selection-constraint writes `selection` and must leave `part_number_inputs` untouched");
    assert!(decoded.script_limits.is_none(), "remove-selection-constraint/drops-the-trailing-length-constraint: remove-selection-constraint writes `selection` and must leave `script_limits` untouched");
    assert!(decoded.artifact.is_none(), "remove-selection-constraint/drops-the-trailing-length-constraint: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "remove-selection-constraint/drops-the-trailing-length-constraint: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the constraint removal, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed remove-selection-constraint diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-selection-constraint/drops-the-trailing-length-constraint: the committed diff did not carry before to after");
}
