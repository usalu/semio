//! 🧪️ `change-pile-base-area-m2` fixture — `🔘️doubles-the-pile-base-area-to-0-5-m2`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-pile-base-area-m2` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1997::{En1997Diff, En1997Mutation, En1997Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1997Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1997Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1997Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-pile-base-area-m2` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Doubling the pile base area from 0.25 m² to 0.5 m² rewrites `pile_base_area_m2` alone — the pile diameter,
/// which a belled base would decouple from the base area anyway, is untouched.
#[semio_framework_async_macros::async_test]
fn doubles_the_pile_base_area_to_0_5_m2() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-pile-base-area-m2 applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.pile_base_area_m2, 0.5, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: pile_base_area_m2 must read 0.5 m² once the change lands");
    assert_eq!(applied.pile_d_m, before().pile_d_m, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: a belled or under-reamed base makes area and shaft diameter genuinely independent, so neither may be derived from the other");
}

/// ↩️ `change-pile-base-area-m2`'s inverse reads the OLD 0.25 m² out of BASE, so replaying it puts the 0.25 m²
/// base area back on `pile_base_area_m2`.
#[semio_framework_async_macros::async_test]
fn restoring_0_25_m2_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-pile-base-area-m2 applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the inverse of one change-pile-base-area-m2 is exactly one change-pile-base-area-m2 back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-pile-base-area-m2 inverse step applies");
    }
    assert_eq!(snapshot.pile_base_area_m2, base.pile_base_area_m2, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the inverse must put the 0.25 m² base area back on `pile_base_area_m2`");
    assert_eq!(snapshot, base, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-pile-base-area-m2` payload are already canonical:
/// decode → encode is a fixed point, so `newPileBaseAreaM2` (serde camelCase over `new_pile_base_area_m2`) is
/// spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-pile-base-area-m2 payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-pile-base-area-m2 payload reparses");
    assert_eq!(reencoded, original, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the committed change-pile-base-area-m2 JSON is not canonical");
}

/// 🎯️ 0.5 m² is finite and differs from the committed 0.25 m², so `change-pile-base-area-m2`
/// emits nothing.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the payload is finite, so `change-pile-base-area-m2`'s `mutation.invariant` fatal cannot fire, and 0.5 differs from the committed 0.25, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: an accepted change-pile-base-area-m2 emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-pile-base-area-m2` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `pileBaseAreaM2` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-pile-base-area-m2 diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the pile base area and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-pile-base-area-m2 diff decodes");
    assert_eq!(decoded.pile_base_area_m2, Some(0.5), "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the committed diff must carry pileBaseAreaM2 = 0.5 m²");
    assert!(decoded.pile_d_m.is_none(), "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: change-pile-base-area-m2 writes pileBaseAreaM2 and must leave `pile_d_m` untouched");
    assert!(decoded.q_b_kpa.is_none(), "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: change-pile-base-area-m2 writes pileBaseAreaM2 and must leave `q_b_kpa` untouched");
    assert!(decoded.artifact.is_none(), "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the base-area change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-pile-base-area-m2 diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: the committed diff did not carry before to after");
    assert_eq!(produced.pile_base_area_m2, 0.5, "change-pile-base-area-m2/doubles-the-pile-base-area-to-0-5-m2: applying the committed diff must land pile_base_area_m2 on 0.5 m²");
}
