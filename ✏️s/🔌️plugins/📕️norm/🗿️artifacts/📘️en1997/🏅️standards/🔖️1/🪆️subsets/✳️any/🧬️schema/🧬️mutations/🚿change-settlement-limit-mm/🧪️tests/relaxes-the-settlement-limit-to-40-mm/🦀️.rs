//! 🧪️ `change-settlement-limit-mm` fixture — `relaxes-the-settlement-limit-to-40-mm`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-settlement-limit-mm` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-settlement-limit-mm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Relaxing the SLS settlement limit from 25.0 mm to 40.0 mm rewrites `settlement_limit_mm` alone — the soil
/// stiffness that determines the COMPUTED settlement is untouched, so the utilisation moves only because the
/// acceptance criterion moved.
#[semio_framework_async_macros::async_test]
fn relaxes_the_settlement_limit_to_40_mm() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-settlement-limit-mm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.settlement_limit_mm, 40.0, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: settlement_limit_mm must read 40.0 mm once the change lands");
    assert_eq!(applied.e_s_mpa, before().e_s_mpa, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the soil modulus drives the computed settlement and must not be traded against the limit it is compared with");
}

/// ↩️ `change-settlement-limit-mm`'s inverse reads the OLD 25.0 mm out of BASE, so replaying it puts the 25.0 mm
/// limit back on `settlement_limit_mm`.
#[semio_framework_async_macros::async_test]
fn restoring_the_25_mm_limit_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-settlement-limit-mm applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the inverse of one change-settlement-limit-mm is exactly one change-settlement-limit-mm back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-settlement-limit-mm inverse step applies");
    }
    assert_eq!(snapshot.settlement_limit_mm, base.settlement_limit_mm, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the inverse must put the 25.0 mm limit back on `settlement_limit_mm`");
    assert_eq!(snapshot, base, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-settlement-limit-mm` payload are already canonical:
/// decode → encode is a fixed point, so `newSettlementLimitMm` (serde camelCase over
/// `new_settlement_limit_mm`) is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-settlement-limit-mm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-settlement-limit-mm payload reparses");
    assert_eq!(reencoded, original, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the committed change-settlement-limit-mm JSON is not canonical");
}

/// 🎯️ 40.0 mm is finite and differs from the committed 25.0 mm, so `change-settlement-limit-mm`
/// returns a message-free outcome.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the payload is finite, so `change-settlement-limit-mm`'s `mutation.invariant` fatal cannot fire, and 40.0 differs from the committed 25.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: an accepted change-settlement-limit-mm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-settlement-limit-mm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `settlementLimitMm` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-settlement-limit-mm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the settlement limit and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-settlement-limit-mm diff decodes");
    assert_eq!(decoded.settlement_limit_mm, Some(40.0), "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the committed diff must carry settlementLimitMm = 40.0 mm");
    assert!(decoded.e_s_mpa.is_none(), "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: change-settlement-limit-mm writes settlementLimitMm and must leave `e_s_mpa` untouched");
    assert!(decoded.nu.is_none(), "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: change-settlement-limit-mm writes settlementLimitMm and must leave `nu` untouched");
    assert!(decoded.artifact.is_none(), "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the settlement-limit change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-settlement-limit-mm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: the committed diff did not carry before to after");
    assert_eq!(produced.settlement_limit_mm, 40.0, "change-settlement-limit-mm/relaxes-the-settlement-limit-to-40-mm: applying the committed diff must land settlement_limit_mm on 40.0 mm");
}
