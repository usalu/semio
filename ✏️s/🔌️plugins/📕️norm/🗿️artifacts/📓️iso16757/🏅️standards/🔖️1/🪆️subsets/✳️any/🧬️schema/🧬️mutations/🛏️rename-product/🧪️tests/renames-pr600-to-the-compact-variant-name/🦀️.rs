//! 🧪️ `rename-product` fixture — `renames-pr600-to-the-compact-variant-name`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `rename-product` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `rename-product` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `rename-product` finds the product by id and writes only its `names.preferred.text`. The product id stays
/// `product.pr600` and the part-number rule still literals `PR-600`, because a marketing name and a part
/// number are different facts.
#[semio_framework_async_macros::async_test]
fn renames_pr600_to_the_compact_variant_name() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("rename-product applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "rename-product/renames-pr600-to-the-compact-variant-name: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.catalogue.products[0].names.preferred.text, "PR-600 Compact", "rename-product/renames-pr600-to-the-compact-variant-name: the product name must change");
    assert_eq!(applied.catalogue.products[0].id, "product.pr600", "rename-product/renames-pr600-to-the-compact-variant-name: the id is the identity and must never follow the label");
    assert_eq!(applied.part_number_rule, before().part_number_rule, "rename-product/renames-pr600-to-the-compact-variant-name: the part-number rule is a different fact from the display name and must not be rewritten");
}

/// ↩️ `rename-product`'s inverse looks the product up in BASE and carries its OLD name, so replaying it puts
/// "PR-600" back on the same id.
#[semio_framework_async_macros::async_test]
fn renaming_the_product_back_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward rename-product applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "rename-product/renames-pr600-to-the-compact-variant-name: the inverse of one product rename is exactly one rename back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the rename-product inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-product/renames-pr600-to-the-compact-variant-name: renaming the product back did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `rename-product` payload are already canonical: decode → encode
/// is a fixed point. The committed payload is spelled `{"RenameProduct": {"id": …, "new_name": …}}` —
/// externally tagged, snake_case payload keys.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "rename-product/renames-pr600-to-the-compact-variant-name: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the rename-product payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the rename-product payload reparses");
    assert_eq!(reencoded, original, "rename-product/renames-pr600-to-the-compact-variant-name: the committed rename-product JSON is not canonical");
}

/// 🎯️ The product exists, so the `mutation.target-missing` Error branch is skipped; and "PR-600 Compact" differs
/// from the committed "PR-600", so the `mutation.no-op` branch is skipped too.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-product/renames-pr600-to-the-compact-variant-name: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "rename-product/renames-pr600-to-the-compact-variant-name: the product exists (no `mutation.target-missing`) and the new name differs from the committed one (no `mutation.no-op`)");
    assert!(produced.messages().is_empty(), "rename-product/renames-pr600-to-the-compact-variant-name: an accepted rename-product emits no diagnostics at all");
}

/// 🔺️ The sparse delta `rename-product` produces is exactly the committed diff — the load-bearing assertion of
/// this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is rewritten and
/// the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced rename-product diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "rename-product/renames-pr600-to-the-compact-variant-name: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed rename-product diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed rename-product diff carries the catalogue");
    assert_eq!(catalogue.products[0].names.preferred.text, "PR-600 Compact", "rename-product/renames-pr600-to-the-compact-variant-name: the diff must carry the new product name");
    assert_eq!(catalogue.products[0].series_id, "series.pr", "rename-product/renames-pr600-to-the-compact-variant-name: the series membership rides through the diff unchanged");
    assert!(decoded.dictionary.is_none(), "rename-product/renames-pr600-to-the-compact-variant-name: rename-product writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "rename-product/renames-pr600-to-the-compact-variant-name: rename-product writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.part_number_rule.is_none(), "rename-product/renames-pr600-to-the-compact-variant-name: rename-product writes `catalogue` and must leave `part_number_rule` untouched");
    assert!(decoded.exchange_process.is_none(), "rename-product/renames-pr600-to-the-compact-variant-name: rename-product writes `catalogue` and must leave `exchange_process` untouched");
    assert!(decoded.artifact.is_none(), "rename-product/renames-pr600-to-the-compact-variant-name: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "rename-product/renames-pr600-to-the-compact-variant-name: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the product rename, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed rename-product diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-product/renames-pr600-to-the-compact-variant-name: the committed diff did not carry before to after");
}
