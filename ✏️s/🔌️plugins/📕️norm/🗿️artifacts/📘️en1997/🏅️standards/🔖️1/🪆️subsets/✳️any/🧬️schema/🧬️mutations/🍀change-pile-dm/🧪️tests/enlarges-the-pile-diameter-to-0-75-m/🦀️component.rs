//! 🧪️ `change-pile-dm` fixture — `enlarges-the-pile-diameter-to-0-75-m`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-pile-dm` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-pile-dm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Enlarging the pile diameter from 0.5 m to 0.75 m rewrites `pile_d_m` alone. The pile BASE AREA is declared
/// separately (0.25 m² here, not πD²/4) and this mutation deliberately does not re-derive it — the two are
/// independent input rows.
#[semio_framework_async_macros::async_test]
async fn enlarges_the_pile_diameter_to_0_75_m() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-pile-dm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.pile_d_m, 0.75, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: pile_d_m must read 0.75 m once the change lands");
    assert_eq!(applied.pile_base_area_m2, before().pile_base_area_m2, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the pile base area is its own declared field and must not be recomputed from the diameter by this mutation");
}

/// ↩️ `change-pile-dm`'s inverse reads the OLD 0.5 m out of BASE, so replaying it puts the 0.5 m pile diameter
/// back on `pile_d_m`.
#[semio_framework_async_macros::async_test]
async fn restoring_0_5_m_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-pile-dm applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the inverse of one change-pile-dm is exactly one change-pile-dm back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-pile-dm inverse step applies");
    }
    assert_eq!(snapshot.pile_d_m, base.pile_d_m, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the inverse must put the 0.5 m pile diameter back on `pile_d_m`");
    assert_eq!(snapshot, base, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-pile-dm` payload are already canonical: decode → encode
/// is a fixed point, so `newPileDM` (serde camelCase over `new_pile_d_m`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-pile-dm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-pile-dm payload reparses");
    assert_eq!(reencoded, original, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the committed change-pile-dm JSON is not canonical");
}

/// 🎯️ 0.75 m is finite and differs from the committed 0.5 m, so `change-pile-dm` (whose guard
/// message reads "Pile diameter [m]") emits nothing.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the payload is finite, so `change-pile-dm`'s `mutation.invariant` fatal cannot fire, and 0.75 differs from the committed 0.5, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: an accepted change-pile-dm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-pile-dm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `pileDM` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-pile-dm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the pile diameter and
/// nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-pile-dm diff decodes");
    assert_eq!(decoded.pile_d_m, Some(0.75), "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the committed diff must carry pileDM = 0.75 m");
    assert!(decoded.pile_base_area_m2.is_none(), "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: change-pile-dm writes pileDM and must leave `pile_base_area_m2` untouched");
    assert!(decoded.pile_l_m.is_none(), "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: change-pile-dm writes pileDM and must leave `pile_l_m` untouched");
    assert!(decoded.artifact.is_none(), "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the pile-diameter change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-pile-dm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: the committed diff did not carry before to after");
    assert_eq!(produced.pile_d_m, 0.75, "change-pile-dm/enlarges-the-pile-diameter-to-0-75-m: applying the committed diff must land pile_d_m on 0.75 m");
}
