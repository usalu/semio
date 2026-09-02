//! 🧪️ `change-exchange-process` fixture — `advances-the-exchange-stage-to-determine-product`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-exchange-process` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `change-exchange-process` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `change-exchange-process` writes the single ISO 16757-5 stage enum, moving the document from
/// `ProvideCatalogue` to `DetermineProduct`. It is the only leaf in this tree whose diff container is a bare
/// scalar rather than a whole collection.
#[semio_framework_async_macros::async_test]
fn advances_the_exchange_stage_to_determine_product() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-exchange-process applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-exchange-process/advances-the-exchange-stage-to-determine-product: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.exchange_process, crate::artifacts::iso16757::part_5::ExchangeProcess::DetermineProduct, "change-exchange-process/advances-the-exchange-stage-to-determine-product: the stage must advance");
    assert_eq!(applied.selection, before().selection, "change-exchange-process/advances-the-exchange-stage-to-determine-product: entering the determine-product stage must not pre-fill the selection request");
    assert_eq!(applied.catalogue, before().catalogue, "change-exchange-process/advances-the-exchange-stage-to-determine-product: nor touch the catalogue being exchanged");
}

/// ↩️ `change-exchange-process`'s inverse reads the OLD stage out of BASE by COPY (the enum is `Copy`, so there
/// is no `.clone()` here), so replaying it returns the document to `ProvideCatalogue`.
#[semio_framework_async_macros::async_test]
fn returning_to_the_provide_catalogue_stage_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-exchange-process applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-exchange-process/advances-the-exchange-stage-to-determine-product: the inverse of one stage change is exactly one stage change back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-exchange-process inverse step applies");
    }
    assert_eq!(snapshot, base, "change-exchange-process/advances-the-exchange-stage-to-determine-product: returning to the provide-catalogue stage did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-exchange-process` payload are already canonical: decode
/// → encode is a fixed point. The committed payload is spelled `{"ChangeExchangeProcess":
/// {"new_exchange_process": "DetermineProduct"}}` — `ExchangeProcess` has `#[dsl(key = "determineProduct")]`
/// for the DSL but NO serde rename, so the JSON spelling is the bare Rust variant name.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "change-exchange-process/advances-the-exchange-stage-to-determine-product: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-exchange-process payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-exchange-process payload reparses");
    assert_eq!(reencoded, original, "change-exchange-process/advances-the-exchange-stage-to-determine-product: the committed change-exchange-process JSON is not canonical");
}

/// 🎯️ `DetermineProduct` differs from the committed `ProvideCatalogue`, so the equality guard stays shut; stage
/// ORDER is not enforced, so any stage may follow any other.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-exchange-process/advances-the-exchange-stage-to-determine-product: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-exchange-process/advances-the-exchange-stage-to-determine-product: the new stage differs from the committed one, so `change-exchange-process`'s `mutation.no-op` guard cannot fire");
    assert!(produced.messages().is_empty(), "change-exchange-process/advances-the-exchange-stage-to-determine-product: an accepted change-exchange-process emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-exchange-process` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only
/// `exchangeProcess` is rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-exchange-process diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-exchange-process/advances-the-exchange-stage-to-determine-product: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the new exchange stage and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed change-exchange-process diff decodes");
    assert_eq!(decoded.exchange_process, Some(crate::artifacts::iso16757::part_5::ExchangeProcess::DetermineProduct), "change-exchange-process/advances-the-exchange-stage-to-determine-product: the diff must carry the new stage");
    assert!(decoded.catalogue.is_none(), "change-exchange-process/advances-the-exchange-stage-to-determine-product: change-exchange-process writes `exchangeProcess` and must leave `catalogue` untouched");
    assert!(decoded.dictionary.is_none(), "change-exchange-process/advances-the-exchange-stage-to-determine-product: change-exchange-process writes `exchangeProcess` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "change-exchange-process/advances-the-exchange-stage-to-determine-product: change-exchange-process writes `exchangeProcess` and must leave `selection` untouched");
    assert!(decoded.script_limits.is_none(), "change-exchange-process/advances-the-exchange-stage-to-determine-product: change-exchange-process writes `exchangeProcess` and must leave `script_limits` untouched");
    assert!(decoded.artifact.is_none(), "change-exchange-process/advances-the-exchange-stage-to-determine-product: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-exchange-process/advances-the-exchange-stage-to-determine-product: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the exchange-stage change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed change-exchange-process diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-exchange-process/advances-the-exchange-stage-to-determine-product: the committed diff did not carry before to after");
}
