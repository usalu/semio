//! 🧪️ `change-part-number-input` fixture — `raises-the-height-part-number-input-to-750`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-part-number-input` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `change-part-number-input` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ The oracle clones the WHOLE `part_number_inputs` map and inserts over the addressed key, so `height` moves
/// to 750.0 while the sibling `length` entry rides through byte-identical — this is an insert-over-clone, not
/// a map replacement.
#[semio_framework_async_macros::async_test]
async fn raises_the_height_part_number_input_to_750() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-part-number-input applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-part-number-input/raises-the-height-part-number-input-to-750: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.part_number_inputs.get("height"), Some(&crate::artifacts::iso16757::CatalogueValue::Decimal { value: 750.0 }), "change-part-number-input/raises-the-height-part-number-input-to-750: the addressed key must hold 750.0");
    assert_eq!(applied.part_number_inputs.get("length"), before().part_number_inputs.get("length"), "change-part-number-input/raises-the-height-part-number-input-to-750: the untargeted `length` input must survive the clone-and-insert unchanged");
    assert_eq!(applied.part_number_inputs.len(), 2, "change-part-number-input/raises-the-height-part-number-input-to-750: writing over an EXISTING key must not grow the map");
}

/// ↩️ `change-part-number-input`'s inverse branches on whether the key already existed: `height` does, so it
/// yields one `ChangePartNumberInput` carrying the OLD 600.0 — not the `RemovePartNumberInput` it would emit
/// for a fresh key.
#[semio_framework_async_macros::async_test]
async fn restoring_the_600_height_input_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-part-number-input applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-part-number-input/raises-the-height-part-number-input-to-750: the existing-key branch of the inverse yields exactly one ChangePartNumberInput back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-part-number-input inverse step applies");
    }
    assert_eq!(snapshot, base, "change-part-number-input/raises-the-height-part-number-input-to-750: replaying the 600.0 value did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-part-number-input` payload are already canonical:
/// decode → encode is a fixed point. The committed payload is spelled `{"ChangePartNumberInput": {"key": …,
/// "new_value": {"kind": "decimal", …}}}` — externally tagged variant, snake_case payload keys, and an
/// internally `kind`-tagged CatalogueValue.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "change-part-number-input/raises-the-height-part-number-input-to-750: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-part-number-input payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-part-number-input payload reparses");
    assert_eq!(reencoded, original, "change-part-number-input/raises-the-height-part-number-input-to-750: the committed change-part-number-input JSON is not canonical");
}

/// 🎯️ The committed `height` input is 600.0, so the `Some(&payload.new_value)` equality guard does not match
/// 750.0 and no `mutation.no-op` warning is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-part-number-input/raises-the-height-part-number-input-to-750: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-part-number-input/raises-the-height-part-number-input-to-750: 750.0 differs from the committed 600.0, so the `base.part_number_inputs.get(key) == Some(&new_value)` guard cannot fire");
    assert!(produced.messages().is_empty(), "change-part-number-input/raises-the-height-part-number-input-to-750: an accepted change-part-number-input emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-part-number-input` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only
/// `partNumberInputs` is rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-part-number-input diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-part-number-input/raises-the-height-part-number-input-to-750: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole rewritten part-
/// number input map and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed change-part-number-input diff decodes");
    let inputs = decoded.part_number_inputs.as_ref().expect("the committed change-part-number-input diff carries the input map");
    assert_eq!(inputs.len(), 2, "change-part-number-input/raises-the-height-part-number-input-to-750: the diff carries BOTH inputs, because this container delta is a whole-map replacement");
    assert_eq!(inputs.get("height"), Some(&crate::artifacts::iso16757::CatalogueValue::Decimal { value: 750.0 }), "change-part-number-input/raises-the-height-part-number-input-to-750: the diff must carry the new 750.0 height");
    assert!(decoded.catalogue.is_none(), "change-part-number-input/raises-the-height-part-number-input-to-750: change-part-number-input writes `partNumberInputs` and must leave `catalogue` untouched");
    assert!(decoded.dictionary.is_none(), "change-part-number-input/raises-the-height-part-number-input-to-750: change-part-number-input writes `partNumberInputs` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "change-part-number-input/raises-the-height-part-number-input-to-750: change-part-number-input writes `partNumberInputs` and must leave `selection` untouched");
    assert!(decoded.part_number_rule.is_none(), "change-part-number-input/raises-the-height-part-number-input-to-750: change-part-number-input writes `partNumberInputs` and must leave `part_number_rule` untouched");
    assert!(decoded.artifact.is_none(), "change-part-number-input/raises-the-height-part-number-input-to-750: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-part-number-input/raises-the-height-part-number-input-to-750: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the part-number input change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed change-part-number-input diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-part-number-input/raises-the-height-part-number-input-to-750: the committed diff did not carry before to after");
}
