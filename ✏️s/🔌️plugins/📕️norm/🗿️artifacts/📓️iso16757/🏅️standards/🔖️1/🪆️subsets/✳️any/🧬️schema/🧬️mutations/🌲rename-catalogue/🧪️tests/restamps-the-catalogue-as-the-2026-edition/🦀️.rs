//! 🧪️ `rename-catalogue` fixture — `restamps-the-catalogue-as-the-2026-edition`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `rename-catalogue` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `rename-catalogue` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `rename-catalogue` writes `catalogue.metadata.names.preferred.text` and nothing else — the German
/// alternative name, the short name, the lifecycle revision and the catalogue ID are all carried through the
/// whole-catalogue clone untouched.
#[semio_framework_async_macros::async_test]
fn restamps_the_catalogue_as_the_2026_edition() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("rename-catalogue applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.catalogue.metadata.names.preferred.text, "Fixture Radiator Catalogue 2026", "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the preferred name must be restamped");
    assert_eq!(
        applied.catalogue.metadata.names.alternatives,
        before().catalogue.metadata.names.alternatives,
        "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the German alternative name is a separate locale entry and must not be rewritten"
    );
    assert_eq!(applied.catalogue.id, before().catalogue.id, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: renaming must never re-mint the catalogue identifier");
}

/// ↩️ `rename-catalogue`'s inverse reads the OLD preferred text out of BASE, so replaying it puts "Fixture
/// Radiator Catalogue" back on the metadata.
#[semio_framework_async_macros::async_test]
fn renaming_back_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward rename-catalogue applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the inverse of one catalogue rename is exactly one rename back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the rename-catalogue inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: renaming back did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `rename-catalogue` payload are already canonical: decode →
/// encode is a fixed point. The committed payload is spelled `{"RenameCatalogue": {"new_name": …}}` —
/// externally tagged, snake_case payload key.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the rename-catalogue payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the rename-catalogue payload reparses");
    assert_eq!(reencoded, original, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the committed rename-catalogue JSON is not canonical");
}

/// 🎯️ "Fixture Radiator Catalogue 2026" differs from the committed "Fixture Radiator Catalogue", so the equality
/// guard on `metadata.names.preferred.text` does not degrade this to a `mutation.no-op` warning.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the new name differs from the committed preferred text, so `rename-catalogue`'s `mutation.no-op` guard cannot fire");
    assert!(produced.messages().is_empty(), "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: an accepted rename-catalogue emits no diagnostics at all");
}

/// 🔺️ The sparse delta `rename-catalogue` produces is exactly the committed diff — the load-bearing assertion of
/// this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is rewritten and
/// the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced rename-catalogue diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed rename-catalogue diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed rename-catalogue diff carries the catalogue");
    assert_eq!(catalogue.metadata.names.preferred.text, "Fixture Radiator Catalogue 2026", "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the diff must carry the new preferred name");
    assert_eq!(catalogue.products.len(), 1, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the catalogue delta is whole-container, so the untouched product list rides along in full");
    assert!(decoded.dictionary.is_none(), "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: rename-catalogue writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: rename-catalogue writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.part_number_inputs.is_none(), "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: rename-catalogue writes `catalogue` and must leave `part_number_inputs` untouched");
    assert!(decoded.exchange_process.is_none(), "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: rename-catalogue writes `catalogue` and must leave `exchange_process` untouched");
    assert!(decoded.artifact.is_none(), "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the catalogue rename, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed rename-catalogue diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-catalogue/restamps-the-catalogue-as-the-2026-edition: the committed diff did not carry before to after");
}
