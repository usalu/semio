//! 🧪️ `create-property-definition` fixture — `appends-a-selection-scoped-length-property`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `create-property-definition` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `create-property-definition` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ With `index: None` the oracle appends, so `prop.length` lands after `prop.height`. It declares
/// `PropertyKind::Selection` and an OPTIONAL cardinality (`min: 0`), which distinguishes it from the
/// committed mandatory `Static` height property.
#[semio_framework_async_macros::async_test]
fn appends_a_selection_scoped_length_property() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("create-property-definition applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-property-definition/appends-a-selection-scoped-length-property: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.catalogue.property_definitions.len(), 2, "create-property-definition/appends-a-selection-scoped-length-property: the definition list must grow by exactly one");
    assert_eq!(applied.catalogue.property_definitions[1].kind, crate::artifacts::iso16757::part_1::PropertyKind::Selection, "create-property-definition/appends-a-selection-scoped-length-property: the new definition must keep its Selection kind");
    assert_eq!(applied.catalogue.property_definitions[1].cardinality.min, 0, "create-property-definition/appends-a-selection-scoped-length-property: the new definition is optional, unlike the committed mandatory height property");
}

/// ↩️ `create-property-definition`'s inverse is a `DeletePropertyDefinition` on the created id — produced only
/// because that id is absent from BASE, which it is here.
#[semio_framework_async_macros::async_test]
fn deleting_the_length_property_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward create-property-definition applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "create-property-definition/appends-a-selection-scoped-length-property: creating a fresh definition inverts to exactly one DeletePropertyDefinition");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the create-property-definition inverse step applies");
    }
    assert_eq!(snapshot, base, "create-property-definition/appends-a-selection-scoped-length-property: deleting the created definition did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `create-property-definition` payload are already canonical:
/// decode → encode is a fixed point. The committed payload is spelled `{"CreatePropertyDefinition":
/// {"property_definition": {…}, "index": null}}` — the nested `unit`/`cardinality`/`kind` keep snake_case
/// field names and bare Rust enum spellings.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "create-property-definition/appends-a-selection-scoped-length-property: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the create-property-definition payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the create-property-definition payload reparses");
    assert_eq!(reencoded, original, "create-property-definition/appends-a-selection-scoped-length-property: the committed create-property-definition JSON is not canonical");
}

/// 🎯️ `prop.length` is not among the committed definitions, so the fatal `mutation.duplicate-id` branch is not
/// taken; `index` is `None`, so `mutation.clamped` is not raised either.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-property-definition/appends-a-selection-scoped-length-property: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "create-property-definition/appends-a-selection-scoped-length-property: the id is fresh (no `mutation.duplicate-id`) and the index is null rather than out of range (no `mutation.clamped`)");
    assert!(produced.messages().is_empty(), "create-property-definition/appends-a-selection-scoped-length-property: an accepted create-property-definition emits no diagnostics at all");
}

/// 🔺️ The sparse delta `create-property-definition` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is
/// rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced create-property-definition diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "create-property-definition/appends-a-selection-scoped-length-property: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed create-property-definition diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed create-property-definition diff carries the catalogue");
    assert_eq!(catalogue.property_definitions.len(), 2, "create-property-definition/appends-a-selection-scoped-length-property: the diff carries both definitions, because the catalogue delta is whole-container");
    assert_eq!(catalogue.property_definitions[1].id, "prop.length", "create-property-definition/appends-a-selection-scoped-length-property: a null index appends, so the new definition is last in the diff too");
    assert!(decoded.dictionary.is_none(), "create-property-definition/appends-a-selection-scoped-length-property: create-property-definition writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "create-property-definition/appends-a-selection-scoped-length-property: create-property-definition writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.part_number_rule.is_none(), "create-property-definition/appends-a-selection-scoped-length-property: create-property-definition writes `catalogue` and must leave `part_number_rule` untouched");
    assert!(decoded.script_limits.is_none(), "create-property-definition/appends-a-selection-scoped-length-property: create-property-definition writes `catalogue` and must leave `script_limits` untouched");
    assert!(decoded.artifact.is_none(), "create-property-definition/appends-a-selection-scoped-length-property: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "create-property-definition/appends-a-selection-scoped-length-property: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the property-definition creation, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed create-property-definition diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-property-definition/appends-a-selection-scoped-length-property: the committed diff did not carry before to after");
}
