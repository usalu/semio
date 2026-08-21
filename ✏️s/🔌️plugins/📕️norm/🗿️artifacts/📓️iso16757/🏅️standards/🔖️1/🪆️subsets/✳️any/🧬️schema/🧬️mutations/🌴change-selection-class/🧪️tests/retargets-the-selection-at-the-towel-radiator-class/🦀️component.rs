//! 🧪️ `change-selection-class` fixture — `retargets-the-selection-at-the-towel-radiator-class`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-selection-class` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `change-selection-class` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ `change-selection-class` clones the `SelectionRequest` and writes only `class_id`. The new id does NOT
/// exist in `catalogue.product_classes`, and the oracle has no referential guard — a selection may
/// legitimately be retargeted before the class is created, so this must apply cleanly.
#[semio_framework_async_macros::async_test]
async fn retargets_the_selection_at_the_towel_radiator_class() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-selection-class applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.selection.class_id, "class.towel-radiator", "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the selection class must be retargeted");
    assert_eq!(applied.selection.constraints, before().selection.constraints, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: both existing constraints must ride through the clone");
    assert_eq!(applied.catalogue.product_classes.len(), 1, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: retargeting at an id that does not exist yet must not create a class");
}

/// ↩️ `change-selection-class`'s inverse reads the OLD `class_id` out of BASE, so replaying it points the
/// request back at `class.panel-radiator`.
#[semio_framework_async_macros::async_test]
async fn retargeting_at_the_panel_radiator_class_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-selection-class applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the inverse of one selection-class change is exactly one change back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-selection-class inverse step applies");
    }
    assert_eq!(snapshot, base, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: pointing the request back at the panel-radiator class did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-selection-class` payload are already canonical: decode
/// → encode is a fixed point. The committed payload is spelled `{"ChangeSelectionClass": {"new_class_id":
/// …}}` — externally tagged, snake_case payload key.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-selection-class payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-selection-class payload reparses");
    assert_eq!(reencoded, original, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the committed change-selection-class JSON is not canonical");
}

/// 🎯️ "class.towel-radiator" differs from the committed "class.panel-radiator", so the equality guard stays
/// shut. There is deliberately no existence check on the target class.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the new class id differs from the committed one, so `change-selection-class`'s `mutation.no-op` guard cannot fire");
    assert!(produced.messages().is_empty(), "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: an accepted change-selection-class emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-selection-class` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `selection` is
/// rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-selection-class diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// selection request and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed change-selection-class diff decodes");
    let selection = decoded.selection.as_ref().expect("the committed change-selection-class diff carries the selection request");
    assert_eq!(selection.class_id, "class.towel-radiator", "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the diff must carry the new class id");
    assert_eq!(selection.constraints.len(), 2, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the selection delta is whole-container, so both constraints ride along");
    assert!(decoded.catalogue.is_none(), "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: change-selection-class writes `selection` and must leave `catalogue` untouched");
    assert!(decoded.dictionary.is_none(), "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: change-selection-class writes `selection` and must leave `dictionary` untouched");
    assert!(decoded.part_number_inputs.is_none(), "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: change-selection-class writes `selection` and must leave `part_number_inputs` untouched");
    assert!(decoded.exchange_process.is_none(), "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: change-selection-class writes `selection` and must leave `exchange_process` untouched");
    assert!(decoded.artifact.is_none(), "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the selection-class change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed change-selection-class diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-selection-class/retargets-the-selection-at-the-towel-radiator-class: the committed diff did not carry before to after");
}
