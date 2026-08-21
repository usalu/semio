//! 🧪️ `change-n-cycles-bridge` fixture — `quadruples-the-bridge-fatigue-cycles-to-2000000`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-n-cycles-bridge` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-n-cycles-bridge` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Quadrupling the design fatigue cycle count from 500000.0 to 2000000.0 rewrites `n_cycles_bridge` alone —
/// the EN 1995-2 fatigue check reads it against the static strengths, none of which this mutation may touch.
#[semio_framework_async_macros::async_test]
async fn quadruples_the_bridge_fatigue_cycles_to_2000000() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-n-cycles-bridge applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.n_cycles_bridge, 2000000.0, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: n_cycles_bridge must read 2000000.0 cycles once the change lands");
    assert_eq!(applied.f_m_k, before().f_m_k, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the characteristic bending strength is the static property the fatigue check degrades from and is not itself a fatigue input");
}

/// ↩️ `change-n-cycles-bridge`'s inverse reads the OLD 500000.0 out of BASE, so replaying it puts the 500000.0
/// cycles back on `n_cycles_bridge`.
#[semio_framework_async_macros::async_test]
async fn restoring_500000_cycles_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-n-cycles-bridge applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the inverse of one change-n-cycles-bridge is exactly one change-n-cycles-bridge back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-n-cycles-bridge inverse step applies");
    }
    assert_eq!(snapshot.n_cycles_bridge, base.n_cycles_bridge, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the inverse must put the 500000.0 cycles back on `n_cycles_bridge`");
    assert_eq!(snapshot, base, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-n-cycles-bridge` payload are already canonical: decode
/// → encode is a fixed point, so `{"ChangeNCyclesBridge": {"newNCyclesBridge": 2000000.0}}` — a JSON FLOAT,
/// because the count is stored as an `f64` is spelled here exactly as this artifact's own serde attributes
/// render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-n-cycles-bridge payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-n-cycles-bridge payload reparses");
    assert_eq!(reencoded, original, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the committed change-n-cycles-bridge JSON is not canonical");
}

/// 🎯️ `n_cycles_bridge` is an `f64`, not an integer count, so `change-n-cycles-bridge` carries a
/// finiteness guard; 2000000.0 is finite and differs from the committed 500000.0.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the payload is finite, so `change-n-cycles-bridge`'s `mutation.invariant` fatal cannot fire, and 2000000.0 differs from the committed 500000.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: an accepted change-n-cycles-bridge emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-n-cycles-bridge` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `nCyclesBridge` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-n-cycles-bridge diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the bridge fatigue cycle
/// count and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-n-cycles-bridge diff decodes");
    assert_eq!(decoded.n_cycles_bridge, Some(2000000.0), "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the committed diff must carry nCyclesBridge = 2000000.0 cycles");
    assert!(decoded.a_vert_m_s2.is_none(), "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: change-n-cycles-bridge writes nCyclesBridge and must leave `a_vert_m_s2` untouched");
    assert!(decoded.f_m_k.is_none(), "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: change-n-cycles-bridge writes nCyclesBridge and must leave `f_m_k` untouched");
    assert!(decoded.artifact.is_none(), "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the fatigue-cycle change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-n-cycles-bridge diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: the committed diff did not carry before to after");
    assert_eq!(produced.n_cycles_bridge, 2000000.0, "change-n-cycles-bridge/quadruples-the-bridge-fatigue-cycles-to-2000000: applying the committed diff must land n_cycles_bridge on 2000000.0 cycles");
}
