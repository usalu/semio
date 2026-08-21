//! 🧪️ `create-product-group` fixture — `appends-a-towel-radiators-group`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `create-product-group` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `create-product-group` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ With `index: None` the oracle appends, so `group.towel-radiators` lands after `group.radiators`. It
/// carries `dictionary_subject_id: None`, i.e. a group may exist before it is mapped onto a dictionary
/// subject.
#[semio_framework_async_macros::async_test]
async fn appends_a_towel_radiators_group() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("create-product-group applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-product-group/appends-a-towel-radiators-group: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.catalogue.product_groups.len(), 2, "create-product-group/appends-a-towel-radiators-group: the group list must grow by exactly one");
    assert_eq!(applied.catalogue.product_groups[1].id, "group.towel-radiators", "create-product-group/appends-a-towel-radiators-group: a null index appends, so the new group must be last");
    assert!(applied.catalogue.product_groups[1].dictionary_subject_id.is_none(), "create-product-group/appends-a-towel-radiators-group: an unmapped group must stay unmapped, not inherit a subject");
}

/// ↩️ `create-product-group`'s inverse is a `DeleteProductGroup` on the created id — produced only because that
/// id is absent from BASE, which it is here.
#[semio_framework_async_macros::async_test]
async fn deleting_the_towel_radiators_group_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward create-product-group applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "create-product-group/appends-a-towel-radiators-group: creating a fresh group inverts to exactly one DeleteProductGroup");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the create-product-group inverse step applies");
    }
    assert_eq!(snapshot, base, "create-product-group/appends-a-towel-radiators-group: deleting the created group did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `create-product-group` payload are already canonical: decode →
/// encode is a fixed point. The committed payload is spelled `{"CreateProductGroup": {"product_group": {…},
/// "index": null}}` — `Names.short_name` and `dictionary_subject_id` are plain `Option`s with no skip
/// attribute, so both appear as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "create-product-group/appends-a-towel-radiators-group: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the create-product-group payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the create-product-group payload reparses");
    assert_eq!(reencoded, original, "create-product-group/appends-a-towel-radiators-group: the committed create-product-group JSON is not canonical");
}

/// 🎯️ `group.towel-radiators` is not among the committed groups, so the fatal `mutation.duplicate-id` branch is
/// not taken; `index` is `None`, so `mutation.clamped` is not raised either.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-product-group/appends-a-towel-radiators-group: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "create-product-group/appends-a-towel-radiators-group: the id is fresh (no `mutation.duplicate-id`) and the index is null rather than out of range (no `mutation.clamped`)");
    assert!(produced.messages().is_empty(), "create-product-group/appends-a-towel-radiators-group: an accepted create-product-group emits no diagnostics at all");
}

/// 🔺️ The sparse delta `create-product-group` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only `catalogue` is
/// rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced create-product-group diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "create-product-group/appends-a-towel-radiators-group: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten
/// catalogue and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed create-product-group diff decodes");
    let catalogue = decoded.catalogue.as_ref().expect("the committed create-product-group diff carries the catalogue");
    assert_eq!(catalogue.product_groups.len(), 2, "create-product-group/appends-a-towel-radiators-group: the diff carries both groups, because the catalogue delta is whole-container");
    assert_eq!(catalogue.product_groups[0].id, "group.radiators", "create-product-group/appends-a-towel-radiators-group: the pre-existing group must keep position 0 in the diff");
    assert!(decoded.dictionary.is_none(), "create-product-group/appends-a-towel-radiators-group: create-product-group writes `catalogue` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "create-product-group/appends-a-towel-radiators-group: create-product-group writes `catalogue` and must leave `selection` untouched");
    assert!(decoded.part_number_inputs.is_none(), "create-product-group/appends-a-towel-radiators-group: create-product-group writes `catalogue` and must leave `part_number_inputs` untouched");
    assert!(decoded.exchange_process.is_none(), "create-product-group/appends-a-towel-radiators-group: create-product-group writes `catalogue` and must leave `exchange_process` untouched");
    assert!(decoded.artifact.is_none(), "create-product-group/appends-a-towel-radiators-group: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "create-product-group/appends-a-towel-radiators-group: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the product-group creation, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed create-product-group diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-product-group/appends-a-towel-radiators-group: the committed diff did not carry before to after");
}
