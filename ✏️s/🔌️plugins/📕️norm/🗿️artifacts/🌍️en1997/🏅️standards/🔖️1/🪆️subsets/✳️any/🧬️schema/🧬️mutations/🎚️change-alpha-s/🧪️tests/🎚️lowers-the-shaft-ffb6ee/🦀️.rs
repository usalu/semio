//! 🧪️ `change-alpha-s` fixture — `🎚️lowers-the-shaft-resistance-factor-to-0-5`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-alpha-s` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-alpha-s` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Lowering α_s from 0.75 to 0.5 rewrites `alpha_s` alone. It scales the mobilised shaft resistance, but the
/// unit shaft resistance q_s it scales is a separate ground-model input and stays as committed.
#[semio_framework_async_macros::async_test]
fn lowers_the_shaft_resistance_factor_to_0_5() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-alpha-s applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.alpha_s, 0.5, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: alpha_s must read 0.5 once the change lands");
    assert_eq!(applied.q_s_kpa, before().q_s_kpa, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the unit shaft resistance is the ground-model quantity α_s multiplies and must not absorb the factor change");
}

/// ↩️ `change-alpha-s`'s inverse reads the OLD 0.75 out of BASE, so replaying it puts the 0.75 shaft-resistance
/// factor back on `alpha_s`.
#[semio_framework_async_macros::async_test]
fn restoring_0_75_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-alpha-s applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the inverse of one change-alpha-s is exactly one change-alpha-s back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-alpha-s inverse step applies");
    }
    assert_eq!(snapshot.alpha_s, base.alpha_s, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the inverse must put the 0.75 shaft-resistance factor back on `alpha_s`");
    assert_eq!(snapshot, base, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-alpha-s` payload are already canonical: decode → encode
/// is a fixed point, so `newAlphaS` (serde camelCase over `new_alpha_s`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-alpha-s payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-alpha-s payload reparses");
    assert_eq!(reencoded, original, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the committed change-alpha-s JSON is not canonical");
}

/// 🎯️ 0.5 is finite and differs from the committed 0.75, so `change-alpha-s` produces a clean
/// outcome.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the payload is finite, so `change-alpha-s`'s `mutation.invariant` fatal cannot fire, and 0.5 differs from the committed 0.75, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: an accepted change-alpha-s emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-alpha-s` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `alphaS` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-alpha-s diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the shaft-resistance factor
/// and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-alpha-s diff decodes");
    assert_eq!(decoded.alpha_s, Some(0.5), "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the committed diff must carry alphaS = 0.5");
    assert!(decoded.q_s_kpa.is_none(), "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: change-alpha-s writes alphaS and must leave `q_s_kpa` untouched");
    assert!(decoded.q_b_kpa.is_none(), "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: change-alpha-s writes alphaS and must leave `q_b_kpa` untouched");
    assert!(decoded.artifact.is_none(), "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the shaft-factor change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-alpha-s diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: the committed diff did not carry before to after");
    assert_eq!(produced.alpha_s, 0.5, "change-alpha-s/lowers-the-shaft-resistance-factor-to-0-5: applying the committed diff must land alpha_s on 0.5");
}
