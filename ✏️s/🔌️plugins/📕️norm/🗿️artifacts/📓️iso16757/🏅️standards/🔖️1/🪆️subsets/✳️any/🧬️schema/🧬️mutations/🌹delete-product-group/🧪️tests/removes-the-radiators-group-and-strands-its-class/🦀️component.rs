//! 🧪️ `delete-product-group` fixture — `removes-the-radiators-group-and-strands-its-class`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `delete-product-group` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `delete-product-group` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `delete-product-group` retains everything whose id differs and performs NO cascade: `class.panel-radiator`
/// keeps pointing at the now-absent `group.radiators`. That stranded reference is deliberate and is what this
/// case pins.
#[semio_framework_async_macros::async_test]
fn removes_the_radiators_group_and_strands_its_class() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("delete-product-group applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: the applied state differs from the committed after-snapshot");
    assert!(applied.catalogue.product_groups.is_empty(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: the addressed group must be gone");
    assert_eq!(applied.catalogue.product_classes[0].group_id, "group.radiators", "delete-product-group/removes-the-radiators-group-and-strands-its-class: the oracle severs nothing, so the class keeps its now-dangling group_id");
    assert_eq!(applied.catalogue.products.len(), 1, "delete-product-group/removes-the-radiators-group-and-strands-its-class: no product is deleted along with the group");
}

/// ↩️ `delete-product-group`'s inverse is a `CreateProductGroup` carrying the removed group AND its recorded
/// index, so the group returns to position 0.
#[semio_framework_async_macros::async_test]
fn recreating_the_radiators_group_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward delete-product-group applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "delete-product-group/removes-the-radiators-group-and-strands-its-class: deleting an existing group inverts to exactly one positioned CreateProductGroup");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the delete-product-group inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-product-group/removes-the-radiators-group-and-strands-its-class: recreating the group at its recorded index did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `delete-product-group` payload are already canonical: decode →
/// encode is a fixed point. The committed payload is spelled `{"DeleteProductGroup": {"id":
/// "group.radiators"}}` — externally tagged, snake_case payload key.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "delete-product-group/removes-the-radiators-group-and-strands-its-class: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the delete-product-group payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the delete-product-group payload reparses");
    assert_eq!(reencoded, original, "delete-product-group/removes-the-radiators-group-and-strands-its-class: the committed delete-product-group JSON is not canonical");
}

/// 🎯️ `group.radiators` IS in the committed catalogue, so the `mutation.target-missing` Error branch is not
/// taken — and no extra message is raised for the class left dangling.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-product-group/removes-the-radiators-group-and-strands-its-class: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "delete-product-group/removes-the-radiators-group-and-strands-its-class: the addressed group exists in the committed catalogue, so `delete-product-group`'s `mutation.target-missing` error cannot fire");
    assert!(produced.messages().is_empty(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: an accepted delete-product-group emits no diagnostics at all");
}

/// 🔺️ The sparse delta `delete-product-group` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is
/// rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced delete-product-group diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "delete-product-group/removes-the-radiators-group-and-strands-its-class: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed delete-product-group diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed delete-product-group diff carries the catalogue");
    assert!(catalogue.product_groups.is_empty(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: the deletion is expressed as the shorter whole group list");
    assert_eq!(catalogue.product_classes.len(), 1, "delete-product-group/removes-the-radiators-group-and-strands-its-class: the class list must ride through the diff untouched");
    assert!(decoded.dictionary.is_none(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: delete-product-group writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: delete-product-group writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.part_number_rule.is_none(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: delete-product-group writes `catalogue` and must leave `part_number_rule` untouched");
    assert!(decoded.exchange_process.is_none(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: delete-product-group writes `catalogue` and must leave `exchange_process` untouched");
    assert!(decoded.artifact.is_none(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "delete-product-group/removes-the-radiators-group-and-strands-its-class: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the product-group deletion, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed delete-product-group diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-product-group/removes-the-radiators-group-and-strands-its-class: the committed diff did not carry before to after");
}
