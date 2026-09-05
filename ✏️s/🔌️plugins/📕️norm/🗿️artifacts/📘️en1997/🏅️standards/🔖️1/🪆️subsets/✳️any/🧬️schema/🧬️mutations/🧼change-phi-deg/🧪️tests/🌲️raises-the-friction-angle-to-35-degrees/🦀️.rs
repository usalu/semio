//! 🧪️ `change-phi-deg` fixture — `🌲️raises-the-friction-angle-to-35-degrees`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-phi-deg` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-phi-deg` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising φ' from 30.0° to 35.0° rewrites `phi_deg` alone — the effective cohesion c', the second Mohr-
/// Coulomb strength parameter, is entered separately and stays at the committed 0.0 kPa.
#[semio_framework_async_macros::async_test]
fn raises_the_friction_angle_to_35_degrees() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-phi-deg applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-phi-deg/raises-the-friction-angle-to-35-degrees: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.phi_deg, 35.0, "change-phi-deg/raises-the-friction-angle-to-35-degrees: phi_deg must read 35.0° once the change lands");
    assert_eq!(applied.c_kpa, before().c_kpa, "change-phi-deg/raises-the-friction-angle-to-35-degrees: c' is the other Mohr-Coulomb parameter and must not be traded against the friction angle by a mutation");
}

/// ↩️ `change-phi-deg`'s inverse reads the OLD 30.0° out of BASE, so replaying it puts the 30.0° friction angle
/// back on `phi_deg`.
#[semio_framework_async_macros::async_test]
fn restoring_30_degrees_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-phi-deg applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-phi-deg/raises-the-friction-angle-to-35-degrees: the inverse of one change-phi-deg is exactly one change-phi-deg back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-phi-deg inverse step applies");
    }
    assert_eq!(snapshot.phi_deg, base.phi_deg, "change-phi-deg/raises-the-friction-angle-to-35-degrees: the inverse must put the 30.0° friction angle back on `phi_deg`");
    assert_eq!(snapshot, base, "change-phi-deg/raises-the-friction-angle-to-35-degrees: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-phi-deg` payload are already canonical: decode → encode
/// is a fixed point, so `newPhiDeg` (serde camelCase over `new_phi_deg`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-phi-deg/raises-the-friction-angle-to-35-degrees: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-phi-deg payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-phi-deg payload reparses");
    assert_eq!(reencoded, original, "change-phi-deg/raises-the-friction-angle-to-35-degrees: the committed change-phi-deg JSON is not canonical");
}

/// 🎯️ 35.0° is finite and differs from the committed 30.0°, so `change-phi-deg` raises neither
/// message. The physical 0–45° range is a checking concern, not a mutation invariant.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-phi-deg/raises-the-friction-angle-to-35-degrees: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-phi-deg/raises-the-friction-angle-to-35-degrees: the payload is finite, so `change-phi-deg`'s `mutation.invariant` fatal cannot fire, and 35.0 differs from the committed 30.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-phi-deg/raises-the-friction-angle-to-35-degrees: an accepted change-phi-deg emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-phi-deg` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `phiDeg` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-phi-deg diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-phi-deg/raises-the-friction-angle-to-35-degrees: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the effective friction angle
/// and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-phi-deg diff decodes");
    assert_eq!(decoded.phi_deg, Some(35.0), "change-phi-deg/raises-the-friction-angle-to-35-degrees: the committed diff must carry phiDeg = 35.0°");
    assert!(decoded.c_kpa.is_none(), "change-phi-deg/raises-the-friction-angle-to-35-degrees: change-phi-deg writes phiDeg and must leave `c_kpa` untouched");
    assert!(decoded.gamma_kn_m3.is_none(), "change-phi-deg/raises-the-friction-angle-to-35-degrees: change-phi-deg writes phiDeg and must leave `gamma_kn_m3` untouched");
    assert!(decoded.artifact.is_none(), "change-phi-deg/raises-the-friction-angle-to-35-degrees: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-phi-deg/raises-the-friction-angle-to-35-degrees: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the friction-angle change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-phi-deg diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-phi-deg/raises-the-friction-angle-to-35-degrees: the committed diff did not carry before to after");
    assert_eq!(produced.phi_deg, 35.0, "change-phi-deg/raises-the-friction-angle-to-35-degrees: applying the committed diff must land phi_deg on 35.0°");
}
