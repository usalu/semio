//! 🧪️ `change-es-mpa` fixture — `stiffens-the-soil-modulus-to-45-mpa`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-es-mpa` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-es-mpa` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Stiffening E_s from 30.0 MPa to 45.0 MPa rewrites `e_s_mpa` alone. Elastic settlement falls in proportion,
/// but the SLS settlement limit it is compared against is a serviceability requirement, not a soil property,
/// and must not move.
#[semio_framework_async_macros::async_test]
async fn stiffens_the_soil_modulus_to_45_mpa() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-es-mpa applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.e_s_mpa, 45.0, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: e_s_mpa must read 45.0 MPa once the change lands");
    assert_eq!(applied.settlement_limit_mm, before().settlement_limit_mm, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the settlement limit is the acceptance criterion of the same SLS check and must never be relaxed by a stiffness edit");
}

/// ↩️ `change-es-mpa`'s inverse reads the OLD 30.0 MPa out of BASE, so replaying it puts the 30.0 MPa soil
/// modulus back on `e_s_mpa`.
#[semio_framework_async_macros::async_test]
async fn restoring_30_mpa_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-es-mpa applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the inverse of one change-es-mpa is exactly one change-es-mpa back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-es-mpa inverse step applies");
    }
    assert_eq!(snapshot.e_s_mpa, base.e_s_mpa, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the inverse must put the 30.0 MPa soil modulus back on `e_s_mpa`");
    assert_eq!(snapshot, base, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-es-mpa` payload are already canonical: decode → encode
/// is a fixed point, so `newESMpa` (serde camelCase over `new_e_s_mpa`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-es-mpa payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-es-mpa payload reparses");
    assert_eq!(reencoded, original, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the committed change-es-mpa JSON is not canonical");
}

/// 🎯️ 45.0 MPa is finite and differs from the committed 30.0 MPa, so `change-es-mpa` returns a
/// clean outcome.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the payload is finite, so `change-es-mpa`'s `mutation.invariant` fatal cannot fire, and 45.0 differs from the committed 30.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: an accepted change-es-mpa emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-es-mpa` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `eSMpa` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-es-mpa diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the soil modulus and nothing
/// else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-es-mpa diff decodes");
    assert_eq!(decoded.e_s_mpa, Some(45.0), "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the committed diff must carry eSMpa = 45.0 MPa");
    assert!(decoded.nu.is_none(), "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: change-es-mpa writes eSMpa and must leave `nu` untouched");
    assert!(decoded.settlement_limit_mm.is_none(), "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: change-es-mpa writes eSMpa and must leave `settlement_limit_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the soil-stiffness change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-es-mpa diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: the committed diff did not carry before to after");
    assert_eq!(produced.e_s_mpa, 45.0, "change-es-mpa/stiffens-the-soil-modulus-to-45-mpa: applying the committed diff must land e_s_mpa on 45.0 MPa");
}
