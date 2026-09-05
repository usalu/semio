//! 🧪️ `change-z-investigated-m` fixture — `🔎️deepens-the-investigated-depth-to-12-m`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-z-investigated-m` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-z-investigated-m` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Deepening the investigated depth from 8.0 m to 12.0 m rewrites `z_investigated_m` alone. It now reaches
/// the committed 12.0 m pile toe exactly, which is the EN 1997-2 depth-adequacy condition — but the pile
/// length itself is a design decision and must not move with the investigation.
#[semio_framework_async_macros::async_test]
fn deepens_the_investigated_depth_to_12_m() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-z-investigated-m applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.z_investigated_m, 12.0, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: z_investigated_m must read 12.0 m once the change lands");
    assert_eq!(applied.pile_l_m, before().pile_l_m, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the pile length is the design quantity the investigated depth is CHECKED against and must not be trimmed to fit it");
}

/// ↩️ `change-z-investigated-m`'s inverse reads the OLD 8.0 m out of BASE, so replaying it puts the 8.0 m
/// investigated depth back on `z_investigated_m`.
#[semio_framework_async_macros::async_test]
fn restoring_the_8_m_investigation_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-z-investigated-m applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the inverse of one change-z-investigated-m is exactly one change-z-investigated-m back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-z-investigated-m inverse step applies");
    }
    assert_eq!(snapshot.z_investigated_m, base.z_investigated_m, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the inverse must put the 8.0 m investigated depth back on `z_investigated_m`");
    assert_eq!(snapshot, base, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-z-investigated-m` payload are already canonical: decode
/// → encode is a fixed point, so `newZInvestigatedM` (serde camelCase over `new_z_investigated_m`) is spelled
/// here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-z-investigated-m payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-z-investigated-m payload reparses");
    assert_eq!(reencoded, original, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the committed change-z-investigated-m JSON is not canonical");
}

/// 🎯️ 12.0 m is finite and differs from the committed 8.0 m, so `change-z-investigated-m` (whose
/// guard message reads "Investigated depth [m]") emits nothing.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the payload is finite, so `change-z-investigated-m`'s `mutation.invariant` fatal cannot fire, and 12.0 differs from the committed 8.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: an accepted change-z-investigated-m emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-z-investigated-m` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `zInvestigatedM` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-z-investigated-m diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the investigated depth and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-z-investigated-m diff decodes");
    assert_eq!(decoded.z_investigated_m, Some(12.0), "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the committed diff must carry zInvestigatedM = 12.0 m");
    assert!(decoded.pile_l_m.is_none(), "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: change-z-investigated-m writes zInvestigatedM and must leave `pile_l_m` untouched");
    assert!(decoded.pile_n_profiles.is_none(), "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: change-z-investigated-m writes zInvestigatedM and must leave `pile_n_profiles` untouched");
    assert!(decoded.artifact.is_none(), "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the investigation-depth change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-z-investigated-m diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: the committed diff did not carry before to after");
    assert_eq!(produced.z_investigated_m, 12.0, "change-z-investigated-m/deepens-the-investigated-depth-to-12-m: applying the committed diff must land z_investigated_m on 12.0 m");
}
