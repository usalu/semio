//! 🧪️ `change-silo-hydraulic-radius-m` fixture — `widens-silo-hydraulic-radius-to-2-25-m` (EN 1991 actions).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1991Snapshot {
    serde_json::from_str(BEFORE).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: mutation decodes")
}

/// ▶️ `change-silo-hydraulic-radius-m` carries `silo_hydraulic_radius_m` from 1.5 to 2.25 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: mutation applies to its committed before-snapshot");
    assert_eq!(produced.silo_hydraulic_radius_m, 2.25, "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: `silo_hydraulic_radius_m` must read 2.25 after the mutation");
    assert_eq!(produced.silo_mu, base.silo_mu, "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: `silo_mu` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `silo_hydraulic_radius_m` (1.5) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: inverse step applies");
    }
    assert_eq!(snapshot.silo_hydraulic_radius_m, base.silo_hydraulic_radius_m, "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: inverse must put `silo_hydraulic_radius_m` back to 1.5");
    assert_eq!(snapshot, base, "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: snapshot reparses");
        assert_eq!(reencoded, original, "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: mutation reparses");
    assert_eq!(reencoded, original, "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 1.5→2.25 edit of `silo_hydraulic_radius_m` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: changing `silo_hydraulic_radius_m` away from 1.5 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `siloHydraulicRadiusM` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().silo_hydraulic_radius_m, Some(2.25), "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: the diff must carry `silo_hydraulic_radius_m` = 2.25");
    assert!(outcome.diff().silo_mu.is_none(), "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: the diff must leave `silo_mu` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: committed diff decodes");
    assert_eq!(produced, committed, "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: committed diff decodes");
    assert_eq!(decoded.silo_hydraulic_radius_m, Some(2.25), "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: the committed diff must name `silo_hydraulic_radius_m` = 2.25");
    let reencoded = serde_json::to_value(&decoded).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: committed diff reparses");
    assert_eq!(reencoded, original, "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 2.25 `silo_hydraulic_radius_m` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: committed diff applies to the before-snapshot");
    assert_eq!(produced.silo_hydraulic_radius_m, 2.25, "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: the committed diff must set `silo_hydraulic_radius_m` to 2.25");
    assert_eq!(produced, expected_after(), "change-silo-hydraulic-radius-m/widens-silo-hydraulic-radius-to-2-25-m: committed diff did not carry before to after");
}
