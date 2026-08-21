//! 🧪️ `change-qb-kpa` fixture — `raises-the-unit-base-resistance-to-3200-kpa`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-qb-kpa` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-qb-kpa` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising q_b from 2500.0 kPa to 3200.0 kPa rewrites `q_b_kpa` alone — the base AREA it is multiplied by to
/// give R_b is a geometry input and is untouched.
#[semio_framework_async_macros::async_test]
async fn raises_the_unit_base_resistance_to_3200_kpa() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-qb-kpa applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.q_b_kpa, 3200.0, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: q_b_kpa must read 3200.0 kPa once the change lands");
    assert_eq!(applied.pile_base_area_m2, before().pile_base_area_m2, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the base area is geometry and the unit base resistance is ground strength; the two are entered separately");
}

/// ↩️ `change-qb-kpa`'s inverse reads the OLD 2500.0 kPa out of BASE, so replaying it puts the 2500.0 kPa back
/// on `q_b_kpa`.
#[semio_framework_async_macros::async_test]
async fn restoring_2500_kpa_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-qb-kpa applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the inverse of one change-qb-kpa is exactly one change-qb-kpa back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-qb-kpa inverse step applies");
    }
    assert_eq!(snapshot.q_b_kpa, base.q_b_kpa, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the inverse must put the 2500.0 kPa back on `q_b_kpa`");
    assert_eq!(snapshot, base, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-qb-kpa` payload are already canonical: decode → encode
/// is a fixed point, so `newQBKpa` (serde camelCase over `new_q_b_kpa`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-qb-kpa payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-qb-kpa payload reparses");
    assert_eq!(reencoded, original, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the committed change-qb-kpa JSON is not canonical");
}

/// 🎯️ 3200.0 kPa is finite and differs from the committed 2500.0 kPa, so `change-qb-kpa` (whose
/// guard message reads "Base resistance q_b [kPa]") produces no diagnostics.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the payload is finite, so `change-qb-kpa`'s `mutation.invariant` fatal cannot fire, and 3200.0 differs from the committed 2500.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: an accepted change-qb-kpa emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-qb-kpa` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `qBKpa` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-qb-kpa diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the unit base resistance and
/// nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-qb-kpa diff decodes");
    assert_eq!(decoded.q_b_kpa, Some(3200.0), "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the committed diff must carry qBKpa = 3200.0 kPa");
    assert!(decoded.pile_base_area_m2.is_none(), "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: change-qb-kpa writes qBKpa and must leave `pile_base_area_m2` untouched");
    assert!(decoded.q_s_kpa.is_none(), "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: change-qb-kpa writes qBKpa and must leave `q_s_kpa` untouched");
    assert!(decoded.artifact.is_none(), "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the base-resistance change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-qb-kpa diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: the committed diff did not carry before to after");
    assert_eq!(produced.q_b_kpa, 3200.0, "change-qb-kpa/raises-the-unit-base-resistance-to-3200-kpa: applying the committed diff must land q_b_kpa on 3200.0 kPa");
}
