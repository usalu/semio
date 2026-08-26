//! 🧪️ `rename-manufacturer` fixture — `adds-the-ag-suffix-to-the-manufacturer`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `rename-manufacturer` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `rename-manufacturer` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `rename-manufacturer` reaches into `catalogue.manufacturer.names.preferred.text` — a DIFFERENT `Names`
/// from the catalogue's own metadata — so the catalogue title must be visibly unaffected. That distinction is
/// what this case pins.
#[semio_framework_async_macros::async_test]
fn adds_the_ag_suffix_to_the_manufacturer() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("rename-manufacturer applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.catalogue.manufacturer.names.preferred.text, "Fixture Heating Works AG", "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the manufacturer name must gain the AG suffix");
    assert_eq!(applied.catalogue.metadata.names.preferred.text, before().catalogue.metadata.names.preferred.text, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the CATALOGUE title is a different Names and must not be touched");
    assert_eq!(applied.catalogue.manufacturer.id, before().catalogue.manufacturer.id, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: a rename must not re-mint the manufacturer id");
}

/// ↩️ `rename-manufacturer`'s inverse reads the OLD manufacturer text out of BASE, so replaying it puts "Fixture
/// Heating Works" back.
#[semio_framework_async_macros::async_test]
fn dropping_the_ag_suffix_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward rename-manufacturer applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the inverse of one manufacturer rename is exactly one rename back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the rename-manufacturer inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: dropping the AG suffix again did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `rename-manufacturer` payload are already canonical: decode →
/// encode is a fixed point. The committed payload is spelled `{"RenameManufacturer": {"new_name": …}}` —
/// externally tagged, snake_case payload key.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the rename-manufacturer payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the rename-manufacturer payload reparses");
    assert_eq!(reencoded, original, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the committed rename-manufacturer JSON is not canonical");
}

/// 🎯️ "Fixture Heating Works AG" differs from the committed "Fixture Heating Works", so the equality guard on
/// the MANUFACTURER's preferred text stays shut.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the new name differs from the committed manufacturer text, so `rename-manufacturer`'s `mutation.no-op` guard cannot fire");
    assert!(produced.messages().is_empty(), "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: an accepted rename-manufacturer emits no diagnostics at all");
}

/// 🔺️ The sparse delta `rename-manufacturer` produces is exactly the committed diff — the load-bearing assertion
/// of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is rewritten
/// and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced rename-manufacturer diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed rename-manufacturer diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed rename-manufacturer diff carries the catalogue");
    assert_eq!(catalogue.manufacturer.names.preferred.text, "Fixture Heating Works AG", "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the diff must carry the new manufacturer name");
    assert_eq!(catalogue.metadata.names.preferred.text, "Fixture Radiator Catalogue", "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the catalogue title must ride through the diff unchanged");
    assert!(decoded.dictionary.is_none(), "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: rename-manufacturer writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: rename-manufacturer writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.script_limits.is_none(), "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: rename-manufacturer writes `catalogue` and must leave `script_limits` untouched");
    assert!(decoded.exchange_process.is_none(), "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: rename-manufacturer writes `catalogue` and must leave `exchange_process` untouched");
    assert!(decoded.artifact.is_none(), "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the manufacturer rename, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed rename-manufacturer diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-manufacturer/adds-the-ag-suffix-to-the-manufacturer: the committed diff did not carry before to after");
}
