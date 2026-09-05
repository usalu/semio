//! 🧪️ `change-gamma-kn-m3` fixture — `⚖️raises-the-soil-unit-weight-to-20-kn-m3`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-gamma-kn-m3` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-gamma-kn-m3` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising γ from 18.0 kN/m³ to 20.0 kN/m³ rewrites `gamma_kn_m3` alone. The overburden q = γ·D_f rises from
/// 27.0 kPa to 30.0 kPa, but the founding depth that multiplies it is untouched.
#[semio_framework_async_macros::async_test]
fn raises_the_soil_unit_weight_to_20_kn_m3() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-gamma-kn-m3 applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.gamma_kn_m3, 20.0, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: gamma_kn_m3 must read 20.0 kN/m³ once the change lands");
    assert_eq!(applied.d_f_m, before().d_f_m, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the founding depth is the other factor in the overburden term and is an independent geometry input");
}

/// ↩️ `change-gamma-kn-m3`'s inverse reads the OLD 18.0 kN/m³ out of BASE, so replaying it puts the 18.0 kN/m³
/// back on `gamma_kn_m3`.
#[semio_framework_async_macros::async_test]
fn restoring_18_kn_m3_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-gamma-kn-m3 applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the inverse of one change-gamma-kn-m3 is exactly one change-gamma-kn-m3 back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-gamma-kn-m3 inverse step applies");
    }
    assert_eq!(snapshot.gamma_kn_m3, base.gamma_kn_m3, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the inverse must put the 18.0 kN/m³ back on `gamma_kn_m3`");
    assert_eq!(snapshot, base, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-gamma-kn-m3` payload are already canonical: decode →
/// encode is a fixed point, so `newGammaKnM3` (serde camelCase over `new_gamma_kn_m3`) is spelled here
/// exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-gamma-kn-m3 payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-gamma-kn-m3 payload reparses");
    assert_eq!(reencoded, original, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the committed change-gamma-kn-m3 JSON is not canonical");
}

/// 🎯️ 20.0 kN/m³ is finite and differs from the committed 18.0 kN/m³, so `change-gamma-kn-m3`
/// produces a message-free outcome.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the payload is finite, so `change-gamma-kn-m3`'s `mutation.invariant` fatal cannot fire, and 20.0 differs from the committed 18.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: an accepted change-gamma-kn-m3 emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-gamma-kn-m3` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `gammaKnM3` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-gamma-kn-m3 diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the soil unit weight and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-gamma-kn-m3 diff decodes");
    assert_eq!(decoded.gamma_kn_m3, Some(20.0), "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the committed diff must carry gammaKnM3 = 20.0 kN/m³");
    assert!(decoded.d_f_m.is_none(), "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: change-gamma-kn-m3 writes gammaKnM3 and must leave `d_f_m` untouched");
    assert!(decoded.phi_deg.is_none(), "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: change-gamma-kn-m3 writes gammaKnM3 and must leave `phi_deg` untouched");
    assert!(decoded.artifact.is_none(), "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the unit-weight change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-gamma-kn-m3 diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: the committed diff did not carry before to after");
    assert_eq!(produced.gamma_kn_m3, 20.0, "change-gamma-kn-m3/raises-the-soil-unit-weight-to-20-kn-m3: applying the committed diff must land gamma_kn_m3 on 20.0 kN/m³");
}
