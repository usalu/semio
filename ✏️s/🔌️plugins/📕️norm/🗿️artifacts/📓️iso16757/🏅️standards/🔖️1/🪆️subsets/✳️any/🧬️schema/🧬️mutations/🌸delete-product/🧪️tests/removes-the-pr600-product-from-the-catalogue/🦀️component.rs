//! 🧪️ `delete-product` fixture — `removes-the-pr600-product-from-the-catalogue`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `delete-product` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `delete-product` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `delete-product` retains everything whose id differs, emptying the product list. The series the product
/// belonged to is NOT deleted with it — the oracle touches only `catalogue.products`.
#[semio_framework_async_macros::async_test]
fn removes_the_pr600_product_from_the_catalogue() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("delete-product applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-product/removes-the-pr600-product-from-the-catalogue: the applied state differs from the committed after-snapshot");
    assert!(applied.catalogue.products.is_empty(), "delete-product/removes-the-pr600-product-from-the-catalogue: the addressed product must be gone");
    assert_eq!(applied.catalogue.product_series.len(), 1, "delete-product/removes-the-pr600-product-from-the-catalogue: deleting a product must not cascade into its series");
    assert_eq!(applied.catalogue.product_groups.len(), 1, "delete-product/removes-the-pr600-product-from-the-catalogue: nor into the group above it");
}

/// ↩️ `delete-product`'s inverse is a `CreateProduct` carrying the removed product AND its recorded position
/// (`index: Some(0)`), so the round trip restores order as well as content.
#[semio_framework_async_macros::async_test]
fn recreating_the_pr600_product_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward delete-product applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "delete-product/removes-the-pr600-product-from-the-catalogue: deleting an existing product inverts to exactly one positioned CreateProduct");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the delete-product inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-product/removes-the-pr600-product-from-the-catalogue: recreating the product at its recorded index did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `delete-product` payload are already canonical: decode → encode
/// is a fixed point. The committed payload is spelled `{"DeleteProduct": {"id": "product.pr600"}}` —
/// externally tagged, snake_case payload key.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "delete-product/removes-the-pr600-product-from-the-catalogue: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the delete-product payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the delete-product payload reparses");
    assert_eq!(reencoded, original, "delete-product/removes-the-pr600-product-from-the-catalogue: the committed delete-product JSON is not canonical");
}

/// 🎯️ `product.pr600` IS in the committed catalogue, so the `mutation.target-missing` Error branch is not taken.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-product/removes-the-pr600-product-from-the-catalogue: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "delete-product/removes-the-pr600-product-from-the-catalogue: the addressed product exists in the committed catalogue, so `delete-product`'s `mutation.target-missing` error cannot fire");
    assert!(produced.messages().is_empty(), "delete-product/removes-the-pr600-product-from-the-catalogue: an accepted delete-product emits no diagnostics at all");
}

/// 🔺️ The sparse delta `delete-product` produces is exactly the committed diff — the load-bearing assertion of
/// this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is rewritten and
/// the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced delete-product diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "delete-product/removes-the-pr600-product-from-the-catalogue: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed delete-product diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed delete-product diff carries the catalogue");
    assert!(catalogue.products.is_empty(), "delete-product/removes-the-pr600-product-from-the-catalogue: the deletion is expressed as the shorter whole product list");
    assert_eq!(catalogue.product_series.len(), 1, "delete-product/removes-the-pr600-product-from-the-catalogue: the untouched series list rides along in the whole-container delta");
    assert!(decoded.dictionary.is_none(), "delete-product/removes-the-pr600-product-from-the-catalogue: delete-product writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "delete-product/removes-the-pr600-product-from-the-catalogue: delete-product writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.part_number_inputs.is_none(), "delete-product/removes-the-pr600-product-from-the-catalogue: delete-product writes `catalogue` and must leave `part_number_inputs` untouched");
    assert!(decoded.script_limits.is_none(), "delete-product/removes-the-pr600-product-from-the-catalogue: delete-product writes `catalogue` and must leave `script_limits` untouched");
    assert!(decoded.artifact.is_none(), "delete-product/removes-the-pr600-product-from-the-catalogue: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "delete-product/removes-the-pr600-product-from-the-catalogue: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the product deletion, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed delete-product diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-product/removes-the-pr600-product-from-the-catalogue: the committed diff did not carry before to after");
}
