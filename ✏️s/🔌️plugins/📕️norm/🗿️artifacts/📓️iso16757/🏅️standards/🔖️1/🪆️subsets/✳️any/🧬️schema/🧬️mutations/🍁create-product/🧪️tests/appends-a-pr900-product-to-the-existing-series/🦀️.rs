//! 🧪️ `create-product` fixture — `appends-a-pr900-product-to-the-existing-series`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `create-product` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `create-product` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ With `index: None` the oracle appends, so `product.pr900` lands after `product.pr600`. It declares
/// `series_id: "series.pr"`, an existing series — but the oracle checks only id UNIQUENESS, never referential
/// validity, so nothing about the series is verified here.
#[semio_framework_async_macros::async_test]
fn appends_a_pr900_product_to_the_existing_series() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("create-product applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-product/appends-a-pr900-product-to-the-existing-series: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.catalogue.products.len(), 2, "create-product/appends-a-pr900-product-to-the-existing-series: the product list must grow by exactly one");
    assert_eq!(applied.catalogue.products[1].id, "product.pr900", "create-product/appends-a-pr900-product-to-the-existing-series: a null index appends, so the new product must be last");
    assert_eq!(applied.catalogue.product_series.len(), 1, "create-product/appends-a-pr900-product-to-the-existing-series: joining an existing series must not duplicate that series");
}

/// ↩️ `create-product`'s inverse is a `DeleteProduct` on the created id — produced only because that id is
/// absent from BASE, which it is here.
#[semio_framework_async_macros::async_test]
fn deleting_the_pr900_product_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward create-product applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "create-product/appends-a-pr900-product-to-the-existing-series: creating a fresh product inverts to exactly one DeleteProduct");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the create-product inverse step applies");
    }
    assert_eq!(snapshot, base, "create-product/appends-a-pr900-product-to-the-existing-series: deleting the created product did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `create-product` payload are already canonical: decode → encode
/// is a fixed point. The committed payload is spelled `{"CreateProduct": {"product": {…}, "index": null}}` —
/// the nested `series_id`/`parameter_domains`/`static_properties` keys stay snake_case, because `Product`
/// carries no `rename_all`.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "create-product/appends-a-pr900-product-to-the-existing-series: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the create-product payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the create-product payload reparses");
    assert_eq!(reencoded, original, "create-product/appends-a-pr900-product-to-the-existing-series: the committed create-product JSON is not canonical");
}

/// 🎯️ `product.pr900` is not among the committed products, so the fatal `mutation.duplicate-id` branch is not
/// taken; `index` is `None`, so `mutation.clamped` is not raised either.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-product/appends-a-pr900-product-to-the-existing-series: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "create-product/appends-a-pr900-product-to-the-existing-series: the id is fresh (no `mutation.duplicate-id`) and the index is null rather than out of range (no `mutation.clamped`)");
    assert!(produced.messages().is_empty(), "create-product/appends-a-pr900-product-to-the-existing-series: an accepted create-product emits no diagnostics at all");
}

/// 🔺️ The sparse delta `create-product` produces is exactly the committed diff — the load-bearing assertion of
/// this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is rewritten and
/// the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced create-product diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "create-product/appends-a-pr900-product-to-the-existing-series: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed create-product diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed create-product diff carries the catalogue");
    assert_eq!(catalogue.products.len(), 2, "create-product/appends-a-pr900-product-to-the-existing-series: the diff carries both products, because the catalogue delta is whole-container");
    assert_eq!(catalogue.products[1].series_id, "series.pr", "create-product/appends-a-pr900-product-to-the-existing-series: the declared series id must survive the diff verbatim");
    assert!(decoded.dictionary.is_none(), "create-product/appends-a-pr900-product-to-the-existing-series: create-product writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "create-product/appends-a-pr900-product-to-the-existing-series: create-product writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.part_number_rule.is_none(), "create-product/appends-a-pr900-product-to-the-existing-series: create-product writes `catalogue` and must leave `part_number_rule` untouched");
    assert!(decoded.script_limits.is_none(), "create-product/appends-a-pr900-product-to-the-existing-series: create-product writes `catalogue` and must leave `script_limits` untouched");
    assert!(decoded.artifact.is_none(), "create-product/appends-a-pr900-product-to-the-existing-series: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "create-product/appends-a-pr900-product-to-the-existing-series: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the product creation, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed create-product diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-product/appends-a-pr900-product-to-the-existing-series: the committed diff did not carry before to after");
}
