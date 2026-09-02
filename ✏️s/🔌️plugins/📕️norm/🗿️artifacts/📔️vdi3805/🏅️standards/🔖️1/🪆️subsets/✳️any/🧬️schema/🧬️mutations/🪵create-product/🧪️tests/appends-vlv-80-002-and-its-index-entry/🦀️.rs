//! 🧪️ `create-product` fixture — `appends-vlv-80-002-and-its-index-entry`.
//!
//! A second catalogue product is appended (`index: null` ⇒ push at the end, no `mutation.clamped` warning), and the persisted `catalog.index` gains the matching entry built by `catalog_index_entry_for` — `dn` 80 read out of the configuration's parameter bag, `tags` from the bilingual title.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Vdi3805Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Vdi3805Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Vdi3805Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn applied() -> Vdi3805Snapshot {
    let base = before();
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &base);
    <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(raised.diff(), &base).expect("create-product applies to its committed before-snapshot")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let snapshot = applied();
    assert_eq!(snapshot.catalog.products.len(), 2, "create-product/appends-vlv-80-002-and-its-index-entry: the catalogue must hold both products");
    assert_eq!(snapshot.catalog.products[1].identity.article_number, "VLV-80-002", "create-product/appends-vlv-80-002-and-its-index-entry: a null insert index must append, not prepend");
    assert_eq!(
        snapshot.index.entries.iter().find(|entry| entry.product_id == "VLV-80-002").and_then(|entry| entry.dn),
        Some(80),
        "create-product/appends-vlv-80-002-and-its-index-entry: the index entry must carry dn 80 extracted from the configuration"
    );
    assert_eq!(snapshot, expected_after(), "create-product/appends-vlv-80-002-and-its-index-entry: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then every step of its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::inverse(&mutation, &base);
    assert!(!inverse.is_empty(), "create-product/appends-vlv-80-002-and-its-index-entry: this mutation changes state, so its inverse must not be empty");
    let mut snapshot = applied();
    for step in &inverse {
        let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(step, &snapshot);
        snapshot = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-product/appends-vlv-80-002-and-its-index-entry: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Vdi3805Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-product/appends-vlv-80-002-and-its-index-entry: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-product/appends-vlv-80-002-and-its-index-entry: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `create-product`'s own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "create-product/appends-vlv-80-002-and-its-index-entry: raised diagnostics differ from the committed 🎯️outcome messages");
    let snapshot = applied();
    match status {
        "applied" => assert_ne!(snapshot, before(), "create-product/appends-vlv-80-002-and-its-index-entry: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "create-product/appends-vlv-80-002-and-its-index-entry: a rejected mutation must leave the snapshot untouched"),
        other => panic!("create-product/appends-vlv-80-002-and-its-index-entry: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields `create-product` is allowed to
/// touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &before());
    let raised_diff = raised.diff();
    assert_eq!(raised_diff.catalog.as_ref().map(|catalog| catalog.products.len()), Some(2), "create-product/appends-vlv-80-002-and-its-index-entry: the diff must publish the catalog with two products");
    assert_eq!(raised_diff.index.as_ref().map(|index| index.entries.len()), Some(2), "create-product/appends-vlv-80-002-and-its-index-entry: catalog.index is persisted state and must be republished in lockstep");
    let produced = serde_json::to_value(raised_diff).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-product/appends-vlv-80-002-and-its-index-entry: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type. Note
/// `Vdi3805Diff` carries no `skip_serializing_if`, so every untouched field must be present as an
/// explicit `null` — and its `Option<Option<u32>>` presence field cannot distinguish "cleared" from
/// "untouched" across a JSON round trip, which is why no case here writes it.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Vdi3805Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-product/appends-vlv-80-002-and-its-index-entry: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `create-product` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Vdi3805Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-product/appends-vlv-80-002-and-its-index-entry: committed diff did not carry before to after");
}
