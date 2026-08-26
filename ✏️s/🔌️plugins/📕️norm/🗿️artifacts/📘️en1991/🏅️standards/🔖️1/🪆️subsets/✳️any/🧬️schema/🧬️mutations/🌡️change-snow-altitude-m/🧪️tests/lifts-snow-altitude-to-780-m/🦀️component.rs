//! 🧪️ `change-snow-altitude-m` fixture — `lifts-snow-altitude-to-780-m` (EN 1991 actions).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1991Snapshot {
    serde_json::from_str(BEFORE).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: mutation decodes")
}

/// ▶️ `change-snow-altitude-m` carries `snow_altitude_m` from 320.0 to 780.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: mutation applies to its committed before-snapshot");
    assert_eq!(produced.snow_altitude_m, 780.0, "change-snow-altitude-m/lifts-snow-altitude-to-780-m: `snow_altitude_m` must read 780.0 after the mutation");
    assert_eq!(produced.en_s_k_kn_m2, base.en_s_k_kn_m2, "change-snow-altitude-m/lifts-snow-altitude-to-780-m: `en_s_k_kn_m2` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-snow-altitude-m/lifts-snow-altitude-to-780-m: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `snow_altitude_m` (320.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: inverse step applies");
    }
    assert_eq!(snapshot.snow_altitude_m, base.snow_altitude_m, "change-snow-altitude-m/lifts-snow-altitude-to-780-m: inverse must put `snow_altitude_m` back to 320.0");
    assert_eq!(snapshot, base, "change-snow-altitude-m/lifts-snow-altitude-to-780-m: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: snapshot reparses");
        assert_eq!(reencoded, original, "change-snow-altitude-m/lifts-snow-altitude-to-780-m: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: mutation reparses");
    assert_eq!(reencoded, original, "change-snow-altitude-m/lifts-snow-altitude-to-780-m: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 320.0→780.0 edit of `snow_altitude_m` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-snow-altitude-m/lifts-snow-altitude-to-780-m: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-snow-altitude-m/lifts-snow-altitude-to-780-m: changing `snow_altitude_m` away from 320.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-snow-altitude-m/lifts-snow-altitude-to-780-m: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `snowAltitudeM` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().snow_altitude_m, Some(780.0), "change-snow-altitude-m/lifts-snow-altitude-to-780-m: the diff must carry `snow_altitude_m` = 780.0");
    assert!(outcome.diff().en_s_k_kn_m2.is_none(), "change-snow-altitude-m/lifts-snow-altitude-to-780-m: the diff must leave `en_s_k_kn_m2` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: committed diff decodes");
    assert_eq!(produced, committed, "change-snow-altitude-m/lifts-snow-altitude-to-780-m: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: committed diff decodes");
    assert_eq!(decoded.snow_altitude_m, Some(780.0), "change-snow-altitude-m/lifts-snow-altitude-to-780-m: the committed diff must name `snow_altitude_m` = 780.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: committed diff reparses");
    assert_eq!(reencoded, original, "change-snow-altitude-m/lifts-snow-altitude-to-780-m: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 780.0 `snow_altitude_m` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-snow-altitude-m/lifts-snow-altitude-to-780-m: committed diff applies to the before-snapshot");
    assert_eq!(produced.snow_altitude_m, 780.0, "change-snow-altitude-m/lifts-snow-altitude-to-780-m: the committed diff must set `snow_altitude_m` to 780.0");
    assert_eq!(produced, expected_after(), "change-snow-altitude-m/lifts-snow-altitude-to-780-m: committed diff did not carry before to after");
}
