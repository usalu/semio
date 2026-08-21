//! 🧪️ `change-qs-kpa` fixture — `raises-the-unit-shaft-resistance-to-120-kpa`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-qs-kpa` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1997::{En1997Diff, En1997Mutation, En1997Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1997Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1997Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1997Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-qs-kpa` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising q_s from 80.0 kPa to 120.0 kPa rewrites `q_s_kpa` alone — the unit BASE resistance q_b is the
/// other half of the EN 1997-1 §7.6.2 pile capacity and is entered independently.
#[semio_framework_async_macros::async_test]
async fn raises_the_unit_shaft_resistance_to_120_kpa() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-qs-kpa applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.q_s_kpa, 120.0, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: q_s_kpa must read 120.0 kPa once the change lands");
    assert_eq!(applied.q_b_kpa, before().q_b_kpa, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: shaft and base resistance are two separate ground-model quantities in the same capacity sum");
}

/// ↩️ `change-qs-kpa`'s inverse reads the OLD 80.0 kPa out of BASE, so replaying it puts the 80.0 kPa back on
/// `q_s_kpa`.
#[semio_framework_async_macros::async_test]
async fn restoring_80_kpa_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-qs-kpa applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the inverse of one change-qs-kpa is exactly one change-qs-kpa back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-qs-kpa inverse step applies");
    }
    assert_eq!(snapshot.q_s_kpa, base.q_s_kpa, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the inverse must put the 80.0 kPa back on `q_s_kpa`");
    assert_eq!(snapshot, base, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-qs-kpa` payload are already canonical: decode → encode
/// is a fixed point, so `newQSKpa` (serde camelCase over `new_q_s_kpa`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-qs-kpa payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-qs-kpa payload reparses");
    assert_eq!(reencoded, original, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the committed change-qs-kpa JSON is not canonical");
}

/// 🎯️ 120.0 kPa is finite and differs from the committed 80.0 kPa, so `change-qs-kpa` (whose guard
/// message reads "Shaft resistance q_s [kPa]") stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the payload is finite, so `change-qs-kpa`'s `mutation.invariant` fatal cannot fire, and 120.0 differs from the committed 80.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: an accepted change-qs-kpa emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-qs-kpa` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `qSKpa` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-qs-kpa diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the unit shaft resistance
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-qs-kpa diff decodes");
    assert_eq!(decoded.q_s_kpa, Some(120.0), "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the committed diff must carry qSKpa = 120.0 kPa");
    assert!(decoded.q_b_kpa.is_none(), "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: change-qs-kpa writes qSKpa and must leave `q_b_kpa` untouched");
    assert!(decoded.alpha_s.is_none(), "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: change-qs-kpa writes qSKpa and must leave `alpha_s` untouched");
    assert!(decoded.artifact.is_none(), "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the shaft-resistance change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-qs-kpa diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: the committed diff did not carry before to after");
    assert_eq!(produced.q_s_kpa, 120.0, "change-qs-kpa/raises-the-unit-shaft-resistance-to-120-kpa: applying the committed diff must land q_s_kpa on 120.0 kPa");
}
