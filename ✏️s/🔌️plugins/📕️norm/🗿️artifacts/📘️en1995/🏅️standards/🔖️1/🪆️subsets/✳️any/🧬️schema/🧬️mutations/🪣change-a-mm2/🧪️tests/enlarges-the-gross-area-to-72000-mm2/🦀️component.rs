//! 🧪️ `change-a-mm2` fixture — `enlarges-the-gross-area-to-72000-mm2`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-a-mm2` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1995Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1995Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1995Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-a-mm2` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Enlarging the gross area from 60000.0 mm² to 72000.0 mm² rewrites `a_mm2` alone — the EFFECTIVE connection
/// area A_ef, which is reduced for fastener holes, is a different quantity and stays at the committed 12000.0
/// mm².
#[semio_framework_async_macros::async_test]
async fn enlarges_the_gross_area_to_72000_mm2() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-a-mm2 applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.a_mm2, 72000.0, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: a_mm2 must read 72000.0 mm² once the change lands");
    assert_eq!(applied.a_ef_mm2, before().a_ef_mm2, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the effective connection area is net of fastener holes and is not a fixed fraction of the gross area");
}

/// ↩️ `change-a-mm2`'s inverse reads the OLD 60000.0 mm² out of BASE, so replaying it puts the 60000.0 mm² back
/// on `a_mm2`.
#[semio_framework_async_macros::async_test]
async fn restoring_60000_mm2_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-a-mm2 applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the inverse of one change-a-mm2 is exactly one change-a-mm2 back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-a-mm2 inverse step applies");
    }
    assert_eq!(snapshot.a_mm2, base.a_mm2, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the inverse must put the 60000.0 mm² back on `a_mm2`");
    assert_eq!(snapshot, base, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-a-mm2` payload are already canonical: decode → encode
/// is a fixed point, so `{"ChangeAMm2": {"newAMm2": 72000.0}}` — externally tagged is spelled here exactly as
/// this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-a-mm2 payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-a-mm2 payload reparses");
    assert_eq!(reencoded, original, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the committed change-a-mm2 JSON is not canonical");
}

/// 🎯️ 72000.0 mm² is finite and differs from the committed 60000.0 mm², so `change-a-mm2` emits
/// no diagnostics.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the payload is finite, so `change-a-mm2`'s `mutation.invariant` fatal cannot fire, and 72000.0 differs from the committed 60000.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: an accepted change-a-mm2 emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-a-mm2` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `aMm2` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-a-mm2 diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the gross cross-sectional
/// area and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-a-mm2 diff decodes");
    assert_eq!(decoded.a_mm2, Some(72000.0), "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the committed diff must carry aMm2 = 72000.0 mm²");
    assert!(decoded.a_ef_mm2.is_none(), "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: change-a-mm2 writes aMm2 and must leave `a_ef_mm2` untouched");
    assert!(decoded.b_mm.is_none(), "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: change-a-mm2 writes aMm2 and must leave `b_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the gross-area change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-a-mm2 diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: the committed diff did not carry before to after");
    assert_eq!(produced.a_mm2, 72000.0, "change-a-mm2/enlarges-the-gross-area-to-72000-mm2: applying the committed diff must land a_mm2 on 72000.0 mm²");
}
