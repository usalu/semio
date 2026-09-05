//! 🧪️ `rename-product-group` fixture — `✏️renames-the-radiators-group-to-panel-radiators`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `rename-product-group` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `rename-product-group` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `rename-product-group` finds the group by id and writes only its `names.preferred.text`; the group's own
/// id and its dictionary-subject mapping are untouched, so a rename never re-keys anything that points at it.
#[semio_framework_async_macros::async_test]
fn renames_the_radiators_group_to_panel_radiators() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("rename-product-group applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "rename-product-group/renames-the-radiators-group-to-panel-radiators: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.catalogue.product_groups[0].names.preferred.text, "Panel radiators", "rename-product-group/renames-the-radiators-group-to-panel-radiators: the group name must change");
    assert_eq!(applied.catalogue.product_groups[0].id, "group.radiators", "rename-product-group/renames-the-radiators-group-to-panel-radiators: the id is the identity and must never follow the label");
    assert_eq!(applied.catalogue.product_classes[0].group_id, "group.radiators", "rename-product-group/renames-the-radiators-group-to-panel-radiators: the child class keeps pointing at the same id");
}

/// ↩️ `rename-product-group`'s inverse looks the group up in BASE and carries its OLD name, so replaying it puts
/// "Radiators" back on the same id.
#[semio_framework_async_macros::async_test]
fn renaming_the_group_back_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward rename-product-group applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "rename-product-group/renames-the-radiators-group-to-panel-radiators: the inverse of one group rename is exactly one rename back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the rename-product-group inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-product-group/renames-the-radiators-group-to-panel-radiators: renaming the group back did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `rename-product-group` payload are already canonical: decode →
/// encode is a fixed point. The committed payload is spelled `{"RenameProductGroup": {"id": …, "new_name":
/// …}}` — externally tagged, snake_case payload keys.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "rename-product-group/renames-the-radiators-group-to-panel-radiators: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the rename-product-group payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the rename-product-group payload reparses");
    assert_eq!(reencoded, original, "rename-product-group/renames-the-radiators-group-to-panel-radiators: the committed rename-product-group JSON is not canonical");
}

/// 🎯️ The group exists, so the `mutation.target-missing` Error branch is skipped; and "Panel radiators" differs
/// from the committed "Radiators", so the `mutation.no-op` branch is skipped too. This leaf is one of the few
/// with BOTH guards.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-product-group/renames-the-radiators-group-to-panel-radiators: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "rename-product-group/renames-the-radiators-group-to-panel-radiators: the group exists (no `mutation.target-missing`) and the new name differs from the committed one (no `mutation.no-op`)");
    assert!(produced.messages().is_empty(), "rename-product-group/renames-the-radiators-group-to-panel-radiators: an accepted rename-product-group emits no diagnostics at all");
}

/// 🔺️ The sparse delta `rename-product-group` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is
/// rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced rename-product-group diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "rename-product-group/renames-the-radiators-group-to-panel-radiators: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed rename-product-group diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed rename-product-group diff carries the catalogue");
    assert_eq!(catalogue.product_groups[0].names.preferred.text, "Panel radiators", "rename-product-group/renames-the-radiators-group-to-panel-radiators: the diff must carry the new group name");
    assert_eq!(catalogue.product_groups[0].dictionary_subject_id.as_deref(), Some("subject.radiator"), "rename-product-group/renames-the-radiators-group-to-panel-radiators: the dictionary mapping rides through the diff unchanged");
    assert!(decoded.dictionary.is_none(), "rename-product-group/renames-the-radiators-group-to-panel-radiators: rename-product-group writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "rename-product-group/renames-the-radiators-group-to-panel-radiators: rename-product-group writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.part_number_inputs.is_none(), "rename-product-group/renames-the-radiators-group-to-panel-radiators: rename-product-group writes `catalogue` and must leave `part_number_inputs` untouched");
    assert!(decoded.script_limits.is_none(), "rename-product-group/renames-the-radiators-group-to-panel-radiators: rename-product-group writes `catalogue` and must leave `script_limits` untouched");
    assert!(decoded.artifact.is_none(), "rename-product-group/renames-the-radiators-group-to-panel-radiators: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "rename-product-group/renames-the-radiators-group-to-panel-radiators: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the product-group rename, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed rename-product-group diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-product-group/renames-the-radiators-group-to-panel-radiators: the committed diff did not carry before to after");
}
