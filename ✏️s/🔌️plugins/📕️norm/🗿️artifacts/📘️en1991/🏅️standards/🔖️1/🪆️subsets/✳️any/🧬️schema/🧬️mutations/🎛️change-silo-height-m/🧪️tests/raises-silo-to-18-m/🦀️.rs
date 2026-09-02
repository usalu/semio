//! 🧪️ `change-silo-height-m` fixture — `raises-silo-to-18-m` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-silo-height-m/raises-silo-to-18-m: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-silo-height-m/raises-silo-to-18-m: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-silo-height-m/raises-silo-to-18-m: mutation decodes")
}

/// ▶️ `change-silo-height-m` carries `silo_height_m` from 12.0 to 18.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-silo-height-m/raises-silo-to-18-m: mutation applies to its committed before-snapshot");
    assert_eq!(produced.silo_height_m, 18.0, "change-silo-height-m/raises-silo-to-18-m: `silo_height_m` must read 18.0 after the mutation");
    assert_eq!(produced.silo_hydraulic_radius_m, base.silo_hydraulic_radius_m, "change-silo-height-m/raises-silo-to-18-m: `silo_hydraulic_radius_m` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-silo-height-m/raises-silo-to-18-m: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `silo_height_m` (12.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-silo-height-m/raises-silo-to-18-m: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-silo-height-m/raises-silo-to-18-m: inverse step applies");
    }
    assert_eq!(snapshot.silo_height_m, base.silo_height_m, "change-silo-height-m/raises-silo-to-18-m: inverse must put `silo_height_m` back to 12.0");
    assert_eq!(snapshot, base, "change-silo-height-m/raises-silo-to-18-m: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-silo-height-m/raises-silo-to-18-m: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-silo-height-m/raises-silo-to-18-m: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-silo-height-m/raises-silo-to-18-m: snapshot reparses");
        assert_eq!(reencoded, original, "change-silo-height-m/raises-silo-to-18-m: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-silo-height-m/raises-silo-to-18-m: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-silo-height-m/raises-silo-to-18-m: mutation reparses");
    assert_eq!(reencoded, original, "change-silo-height-m/raises-silo-to-18-m: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 12.0→18.0 edit of `silo_height_m` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-silo-height-m/raises-silo-to-18-m: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-silo-height-m/raises-silo-to-18-m: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-silo-height-m/raises-silo-to-18-m: changing `silo_height_m` away from 12.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-silo-height-m/raises-silo-to-18-m: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `siloHeightM` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().silo_height_m, Some(18.0), "change-silo-height-m/raises-silo-to-18-m: the diff must carry `silo_height_m` = 18.0");
    assert!(outcome.diff().silo_hydraulic_radius_m.is_none(), "change-silo-height-m/raises-silo-to-18-m: the diff must leave `silo_hydraulic_radius_m` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-silo-height-m/raises-silo-to-18-m: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-height-m/raises-silo-to-18-m: committed diff decodes");
    assert_eq!(produced, committed, "change-silo-height-m/raises-silo-to-18-m: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-silo-height-m/raises-silo-to-18-m: committed diff decodes");
    assert_eq!(decoded.silo_height_m, Some(18.0), "change-silo-height-m/raises-silo-to-18-m: the committed diff must name `silo_height_m` = 18.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-silo-height-m/raises-silo-to-18-m: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-height-m/raises-silo-to-18-m: committed diff reparses");
    assert_eq!(reencoded, original, "change-silo-height-m/raises-silo-to-18-m: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 18.0 `silo_height_m` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-silo-height-m/raises-silo-to-18-m: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-silo-height-m/raises-silo-to-18-m: committed diff applies to the before-snapshot");
    assert_eq!(produced.silo_height_m, 18.0, "change-silo-height-m/raises-silo-to-18-m: the committed diff must set `silo_height_m` to 18.0");
    assert_eq!(produced, expected_after(), "change-silo-height-m/raises-silo-to-18-m: committed diff did not carry before to after");
}
