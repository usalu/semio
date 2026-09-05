//! 🧪️ `remove-part-number-input` fixture — `🔢️drops-the-length-part-number-input`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `Iso16757Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `remove-part-number-input` never writes it, so it stays `None` and rides the JSON round trip as a plain `null`;
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
    serde_json::from_str(MUTATION).expect("the committed `remove-part-number-input` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ The oracle clones the map and removes the addressed key, so `length` disappears and `height` survives —
/// the diff is still the whole remaining map, which is why the `after` map is one entry shorter rather than
/// carrying a tombstone.
#[semio_framework_async_macros::async_test]
fn drops_the_length_part_number_input() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("remove-part-number-input applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "remove-part-number-input/drops-the-length-part-number-input: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.part_number_inputs.len(), 1, "remove-part-number-input/drops-the-length-part-number-input: exactly one input must remain");
    assert!(!applied.part_number_inputs.contains_key("length"), "remove-part-number-input/drops-the-length-part-number-input: the addressed key must be gone");
    assert_eq!(applied.part_number_inputs.get("height"), before().part_number_inputs.get("height"), "remove-part-number-input/drops-the-length-part-number-input: the untargeted `height` input must survive");
}

/// ↩️ `remove-part-number-input`'s inverse reads the OLD value out of BASE and yields one
/// `ChangePartNumberInput`, which re-inserts it; because `part_number_inputs` is a `BTreeMap`, re-inserting
/// also restores the key ORDER, not just the contents.
#[semio_framework_async_macros::async_test]
fn reinserting_the_length_input_restores_before() {
    let base = before();
    let forward = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward remove-part-number-input applies");
    let inverse = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "remove-part-number-input/drops-the-length-part-number-input: removing an existing key inverts to exactly one ChangePartNumberInput that puts it back");
    for step in &inverse {
        let undo = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the remove-part-number-input inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-part-number-input/drops-the-length-part-number-input: re-inserting `length` did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `remove-part-number-input` payload are already canonical:
/// decode → encode is a fixed point. The committed payload is spelled `{"RemovePartNumberInput": {"key":
/// "length"}}` — externally tagged, snake_case payload key.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).expect("the committed catalogue snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed catalogue snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed catalogue snapshot reparses");
        assert_eq!(reencoded, original, "remove-part-number-input/drops-the-length-part-number-input: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the remove-part-number-input payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the remove-part-number-input payload reparses");
    assert_eq!(reencoded, original, "remove-part-number-input/drops-the-length-part-number-input: the committed remove-part-number-input JSON is not canonical");
}

/// 🎯️ `length` IS present in the committed inputs, so the `contains_key` guard passes and the `mutation.target-
/// missing` Error branch is not taken.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-part-number-input/drops-the-length-part-number-input: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "remove-part-number-input/drops-the-length-part-number-input: the committed map contains `length`, so `remove-part-number-input`'s `mutation.target-missing` error branch cannot fire");
    assert!(produced.messages().is_empty(), "remove-part-number-input/drops-the-length-part-number-input: an accepted remove-part-number-input emits no diagnostics at all");
}

/// 🔺️ The sparse delta `remove-part-number-input` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: `Iso16757Diff` is a per-CONTAINER delta, so this pins that only
/// `partNumberInputs` is rewritten and the other eight containers stay `null`.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced remove-part-number-input diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "remove-part-number-input/drops-the-length-part-number-input: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `Iso16757Diff`, re-encodes unchanged, and carries the whole surviving part-
/// number input map and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed remove-part-number-input diff decodes");
    let inputs = decoded.part_number_inputs.as_ref().expect("the committed remove-part-number-input diff carries the input map");
    assert_eq!(inputs.len(), 1, "remove-part-number-input/drops-the-length-part-number-input: a removal is expressed as the SHORTER whole map, never as a delete marker");
    assert!(!inputs.contains_key("length"), "remove-part-number-input/drops-the-length-part-number-input: the removed key must not appear in the diff");
    assert!(decoded.catalogue.is_none(), "remove-part-number-input/drops-the-length-part-number-input: remove-part-number-input writes `partNumberInputs` and must leave `catalogue` untouched");
    assert!(decoded.dictionary.is_none(), "remove-part-number-input/drops-the-length-part-number-input: remove-part-number-input writes `partNumberInputs` and must leave `dictionary` untouched");
    assert!(decoded.selection.is_none(), "remove-part-number-input/drops-the-length-part-number-input: remove-part-number-input writes `partNumberInputs` and must leave `selection` untouched");
    assert!(decoded.script_limits.is_none(), "remove-part-number-input/drops-the-length-part-number-input: remove-part-number-input writes `partNumberInputs` and must leave `script_limits` untouched");
    assert!(decoded.artifact.is_none(), "remove-part-number-input/drops-the-length-part-number-input: a container-scoped mutation must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "remove-part-number-input/drops-the-length-part-number-input: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete description
/// of the part-number input removal, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).expect("the committed remove-part-number-input diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-part-number-input/drops-the-length-part-number-input: the committed diff did not carry before to after");
}
