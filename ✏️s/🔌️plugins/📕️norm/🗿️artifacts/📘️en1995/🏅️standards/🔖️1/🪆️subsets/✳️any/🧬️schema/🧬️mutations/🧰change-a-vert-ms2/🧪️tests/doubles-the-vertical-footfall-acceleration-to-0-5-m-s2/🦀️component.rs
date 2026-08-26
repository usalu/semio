//! 🧪️ `change-a-vert-ms2` fixture — `doubles-the-vertical-footfall-acceleration-to-0-5-m-s2`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-a-vert-ms2` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-a-vert-ms2` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Doubling the vertical footfall acceleration from 0.25 m/s² to 0.5 m/s² rewrites `a_vert_m_s2` alone — it
/// is the EN 1995-2 pedestrian-comfort criterion and touches none of the EN 1995-1-1 strength inputs.
#[semio_framework_async_macros::async_test]
fn doubles_the_vertical_footfall_acceleration_to_0_5_m_s2() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-a-vert-ms2 applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.a_vert_m_s2, 0.5, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: a_vert_m_s2 must read 0.5 m/s² once the change lands");
    assert_eq!(applied.n_cycles_bridge, before().n_cycles_bridge, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the fatigue cycle count is the other EN 1995-2 bridge quantity and is entered independently of comfort");
}

/// ↩️ `change-a-vert-ms2`'s inverse reads the OLD 0.25 m/s² out of BASE, so replaying it puts the 0.25 m/s²
/// acceleration back on `a_vert_m_s2`.
#[semio_framework_async_macros::async_test]
fn restoring_0_25_m_s2_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-a-vert-ms2 applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the inverse of one change-a-vert-ms2 is exactly one change-a-vert-ms2 back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-a-vert-ms2 inverse step applies");
    }
    assert_eq!(snapshot.a_vert_m_s2, base.a_vert_m_s2, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the inverse must put the 0.25 m/s² acceleration back on `a_vert_m_s2`");
    assert_eq!(snapshot, base, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-a-vert-ms2` payload are already canonical: decode →
/// encode is a fixed point, so `{"ChangeAVertMS2": {"newAVertMS2": 0.5}}` — serde camelCase over
/// `new_a_vert_m_s2` gives `newAVertMS2`, with the trailing `s2` segment capitalised to `S2` is spelled here
/// exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-a-vert-ms2 payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-a-vert-ms2 payload reparses");
    assert_eq!(reencoded, original, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the committed change-a-vert-ms2 JSON is not canonical");
}

/// 🎯️ 0.5 m/s² is finite and differs from the committed 0.25 m/s², so `change-a-vert-ms2` (whose
/// guard message reads "A vert ms2") emits nothing.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the payload is finite, so `change-a-vert-ms2`'s `mutation.invariant` fatal cannot fire, and 0.5 differs from the committed 0.25, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: an accepted change-a-vert-ms2 emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-a-vert-ms2` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `aVertMS2` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-a-vert-ms2 diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the vertical footfall
/// acceleration and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-a-vert-ms2 diff decodes");
    assert_eq!(decoded.a_vert_m_s2, Some(0.5), "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the committed diff must carry aVertMS2 = 0.5 m/s²");
    assert!(decoded.n_cycles_bridge.is_none(), "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: change-a-vert-ms2 writes aVertMS2 and must leave `n_cycles_bridge` untouched");
    assert!(decoded.m_ed_knm.is_none(), "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: change-a-vert-ms2 writes aVertMS2 and must leave `m_ed_knm` untouched");
    assert!(decoded.artifact.is_none(), "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the comfort-criterion change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-a-vert-ms2 diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: the committed diff did not carry before to after");
    assert_eq!(produced.a_vert_m_s2, 0.5, "change-a-vert-ms2/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2: applying the committed diff must land a_vert_m_s2 on 0.5 m/s²");
}
