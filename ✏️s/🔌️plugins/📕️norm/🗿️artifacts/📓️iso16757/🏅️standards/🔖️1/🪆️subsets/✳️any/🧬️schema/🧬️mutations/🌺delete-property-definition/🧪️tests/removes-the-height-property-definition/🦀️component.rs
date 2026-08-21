//! 🧪️ `delete-property-definition` fixture — `removes-the-height-property-definition`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `delete-property-definition` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `delete-property-definition` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `delete-property-definition` empties `catalogue.property_definitions` here. Both `class.panel-
/// radiator.required_property_ids` and the selection's height constraint still name `prop.height`; neither is
/// cleaned up, because the oracle touches only the definition list.
#[semio_framework_async_macros::async_test]
async fn removes_the_height_property_definition() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("delete-property-definition applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-property-definition/removes-the-height-property-definition: the applied state differs from the committed after-snapshot");
    assert!(applied.catalogue.property_definitions.is_empty(), "delete-property-definition/removes-the-height-property-definition: the addressed definition must be gone");
    assert_eq!(applied.catalogue.product_classes[0].required_property_ids, vec!["prop.height".to_string()], "delete-property-definition/removes-the-height-property-definition: the class keeps requiring a property that no longer exists — no cascade");
    assert_eq!(applied.selection.constraints[0].property_id, "prop.height", "delete-property-definition/removes-the-height-property-definition: the selection constraint on the deleted property survives too");
}

/// ↩️ `delete-property-definition`'s inverse is a `CreatePropertyDefinition` carrying the removed definition AND
/// its recorded index, so it returns to position 0 with its unit and cardinality intact.
#[semio_framework_async_macros::async_test]
async fn recreating_the_height_property_definition_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward delete-property-definition applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "delete-property-definition/removes-the-height-property-definition: deleting an existing definition inverts to exactly one positioned CreatePropertyDefinition");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the delete-property-definition inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-property-definition/removes-the-height-property-definition: recreating the definition at its recorded index did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `delete-property-definition` payload are already canonical:
/// decode → encode is a fixed point. The committed payload is spelled `{"DeletePropertyDefinition": {"id":
/// "prop.height"}}` — externally tagged, snake_case payload key.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "delete-property-definition/removes-the-height-property-definition: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the delete-property-definition payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the delete-property-definition payload reparses");
    assert_eq!(reencoded, original, "delete-property-definition/removes-the-height-property-definition: the committed delete-property-definition JSON is not canonical");
}

/// 🎯️ `prop.height` IS in the committed catalogue, so the `mutation.target-missing` Error branch is not taken.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-property-definition/removes-the-height-property-definition: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "delete-property-definition/removes-the-height-property-definition: the addressed definition exists in the committed catalogue, so `delete-property-definition`'s `mutation.target-missing` error cannot fire");
    assert!(produced.messages().is_empty(), "delete-property-definition/removes-the-height-property-definition: an accepted delete-property-definition emits no diagnostics at all");
}

/// 🔺️ The sparse delta `delete-property-definition` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is
/// rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced delete-property-definition diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "delete-property-definition/removes-the-height-property-definition: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed delete-property-definition diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed delete-property-definition diff carries the catalogue");
    assert!(catalogue.property_definitions.is_empty(), "delete-property-definition/removes-the-height-property-definition: the deletion is expressed as the shorter whole definition list");
    assert_eq!(catalogue.product_classes[0].required_property_ids.len(), 1, "delete-property-definition/removes-the-height-property-definition: the class requirement list is untouched inside the same delta");
    assert!(decoded.dictionary.is_none(), "delete-property-definition/removes-the-height-property-definition: delete-property-definition writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "delete-property-definition/removes-the-height-property-definition: delete-property-definition writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.part_number_inputs.is_none(), "delete-property-definition/removes-the-height-property-definition: delete-property-definition writes `catalogue` and must leave `part_number_inputs` untouched");
    assert!(decoded.exchange_process.is_none(), "delete-property-definition/removes-the-height-property-definition: delete-property-definition writes `catalogue` and must leave `exchange_process` untouched");
    assert!(decoded.artifact.is_none(), "delete-property-definition/removes-the-height-property-definition: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "delete-property-definition/removes-the-height-property-definition: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the property-definition deletion, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed delete-property-definition diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-property-definition/removes-the-height-property-definition: the committed diff did not carry before to after");
}
