//! 🧪️ `change-footing-area-m2` fixture — `enlarges-the-footing-area-to-6-25-m2`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-footing-area-m2` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-footing-area-m2` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Enlarging the footing from 4.0 m² to 6.25 m² rewrites `footing_area_m2` alone. B is the OTHER, separately
/// declared footing dimension (2.5 m would be the matching square side) and this mutation deliberately does
/// not derive it.
#[semio_framework_async_macros::async_test]
fn enlarges_the_footing_area_to_6_25_m2() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-footing-area-m2 applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.footing_area_m2, 6.25, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: footing_area_m2 must read 6.25 m² once the change lands");
    assert_eq!(applied.b_m, before().b_m, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the footing width B drives the bearing-capacity Nγ term and is its own input row, never recomputed from the area");
}

/// ↩️ `change-footing-area-m2`'s inverse reads the OLD 4.0 m² out of BASE, so replaying it puts the 4.0 m² back
/// on `footing_area_m2`.
#[semio_framework_async_macros::async_test]
fn restoring_4_m2_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-footing-area-m2 applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the inverse of one change-footing-area-m2 is exactly one change-footing-area-m2 back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-footing-area-m2 inverse step applies");
    }
    assert_eq!(snapshot.footing_area_m2, base.footing_area_m2, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the inverse must put the 4.0 m² back on `footing_area_m2`");
    assert_eq!(snapshot, base, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-footing-area-m2` payload are already canonical: decode
/// → encode is a fixed point, so `newFootingAreaM2` (serde camelCase over `new_footing_area_m2`) is spelled
/// here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-footing-area-m2 payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-footing-area-m2 payload reparses");
    assert_eq!(reencoded, original, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the committed change-footing-area-m2 JSON is not canonical");
}

/// 🎯️ 6.25 m² is finite and differs from the committed 4.0 m², so `change-footing-area-m2` emits
/// no diagnostics.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the payload is finite, so `change-footing-area-m2`'s `mutation.invariant` fatal cannot fire, and 6.25 differs from the committed 4.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: an accepted change-footing-area-m2 emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-footing-area-m2` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `footingAreaM2` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-footing-area-m2 diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the footing area and nothing
/// else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-footing-area-m2 diff decodes");
    assert_eq!(decoded.footing_area_m2, Some(6.25), "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the committed diff must carry footingAreaM2 = 6.25 m²");
    assert!(decoded.b_m.is_none(), "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: change-footing-area-m2 writes footingAreaM2 and must leave `b_m` untouched");
    assert!(decoded.d_f_m.is_none(), "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: change-footing-area-m2 writes footingAreaM2 and must leave `d_f_m` untouched");
    assert!(decoded.artifact.is_none(), "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the footing-area change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-footing-area-m2 diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: the committed diff did not carry before to after");
    assert_eq!(produced.footing_area_m2, 6.25, "change-footing-area-m2/enlarges-the-footing-area-to-6-25-m2: applying the committed diff must land footing_area_m2 on 6.25 m²");
}
