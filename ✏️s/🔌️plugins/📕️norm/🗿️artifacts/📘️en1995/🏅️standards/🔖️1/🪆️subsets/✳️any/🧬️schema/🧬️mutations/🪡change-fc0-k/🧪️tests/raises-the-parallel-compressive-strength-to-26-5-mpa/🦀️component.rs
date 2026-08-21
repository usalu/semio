//! 🧪️ `change-fc0-k` fixture — `raises-the-parallel-compressive-strength-to-26-5-mpa`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-fc0-k` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-fc0-k` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising f_c,0,k from 21.0 MPa to 26.5 MPa rewrites `f_c_0_k` alone — the bending strength that shares the
/// §6.2.4 interaction expression with it is a separate declared property.
#[semio_framework_async_macros::async_test]
async fn raises_the_parallel_compressive_strength_to_26_5_mpa() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-fc0-k applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.f_c_0_k, 26.5, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: f_c_0_k must read 26.5 MPa once the change lands");
    assert_eq!(applied.f_m_k, before().f_m_k, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: f_m,k is the other declared strength in the same interaction and must not move with it");
}

/// ↩️ `change-fc0-k`'s inverse reads the OLD 21.0 MPa out of BASE, so replaying it puts the 21.0 MPa compressive
/// strength back on `f_c_0_k`.
#[semio_framework_async_macros::async_test]
async fn restoring_21_mpa_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-fc0-k applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the inverse of one change-fc0-k is exactly one change-fc0-k back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-fc0-k inverse step applies");
    }
    assert_eq!(snapshot.f_c_0_k, base.f_c_0_k, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the inverse must put the 21.0 MPa compressive strength back on `f_c_0_k`");
    assert_eq!(snapshot, base, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-fc0-k` payload are already canonical: decode → encode
/// is a fixed point, so `{"ChangeFC0K": {"newFC0K": 26.5}}` — serde's PascalCase pass uppercases each
/// underscore-separated segment, and the digit segment `0` survives untouched is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-fc0-k payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-fc0-k payload reparses");
    assert_eq!(reencoded, original, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the committed change-fc0-k JSON is not canonical");
}

/// 🎯️ 26.5 MPa is finite and differs from the committed 21.0 MPa, so `change-fc0-k` (whose guard
/// message reads "Fc0 k") stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the payload is finite, so `change-fc0-k`'s `mutation.invariant` fatal cannot fire, and 26.5 differs from the committed 21.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: an accepted change-fc0-k emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-fc0-k` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `fC0K` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-fc0-k diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the parallel-to-grain
/// compressive strength and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-fc0-k diff decodes");
    assert_eq!(decoded.f_c_0_k, Some(26.5), "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the committed diff must carry fC0K = 26.5 MPa");
    assert!(decoded.f_m_k.is_none(), "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: change-fc0-k writes fC0K and must leave `f_m_k` untouched");
    assert!(decoded.a_mm2.is_none(), "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: change-fc0-k writes fC0K and must leave `a_mm2` untouched");
    assert!(decoded.artifact.is_none(), "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the compressive-strength change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-fc0-k diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: the committed diff did not carry before to after");
    assert_eq!(produced.f_c_0_k, 26.5, "change-fc0-k/raises-the-parallel-compressive-strength-to-26-5-mpa: applying the committed diff must land f_c_0_k on 26.5 MPa");
}
