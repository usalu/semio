//! 🧪️ `change-fire-resistance-min` fixture — `⏱️extends-fire-resistance-to-120-min` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: mutation decodes")
}

/// ▶️ `change-fire-resistance-min` carries `fire_resistance_min` from 90.0 to 120.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: mutation applies to its committed before-snapshot");
    assert_eq!(produced.fire_resistance_min, 120.0, "change-fire-resistance-min/extends-fire-resistance-to-120-min: `fire_resistance_min` must read 120.0 after the mutation");
    assert_eq!(produced.fire_member_capacity_c, base.fire_member_capacity_c, "change-fire-resistance-min/extends-fire-resistance-to-120-min: `fire_member_capacity_c` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-fire-resistance-min/extends-fire-resistance-to-120-min: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `fire_resistance_min` (90.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: inverse step applies");
    }
    assert_eq!(snapshot.fire_resistance_min, base.fire_resistance_min, "change-fire-resistance-min/extends-fire-resistance-to-120-min: inverse must put `fire_resistance_min` back to 90.0");
    assert_eq!(snapshot, base, "change-fire-resistance-min/extends-fire-resistance-to-120-min: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: snapshot reparses");
        assert_eq!(reencoded, original, "change-fire-resistance-min/extends-fire-resistance-to-120-min: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: mutation reparses");
    assert_eq!(reencoded, original, "change-fire-resistance-min/extends-fire-resistance-to-120-min: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 90.0→120.0 edit of `fire_resistance_min` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fire-resistance-min/extends-fire-resistance-to-120-min: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-fire-resistance-min/extends-fire-resistance-to-120-min: changing `fire_resistance_min` away from 90.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-fire-resistance-min/extends-fire-resistance-to-120-min: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `fireResistanceMin` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().fire_resistance_min, Some(120.0), "change-fire-resistance-min/extends-fire-resistance-to-120-min: the diff must carry `fire_resistance_min` = 120.0");
    assert!(outcome.diff().fire_member_capacity_c.is_none(), "change-fire-resistance-min/extends-fire-resistance-to-120-min: the diff must leave `fire_member_capacity_c` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: committed diff decodes");
    assert_eq!(produced, committed, "change-fire-resistance-min/extends-fire-resistance-to-120-min: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: committed diff decodes");
    assert_eq!(decoded.fire_resistance_min, Some(120.0), "change-fire-resistance-min/extends-fire-resistance-to-120-min: the committed diff must name `fire_resistance_min` = 120.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: committed diff reparses");
    assert_eq!(reencoded, original, "change-fire-resistance-min/extends-fire-resistance-to-120-min: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 120.0 `fire_resistance_min` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-fire-resistance-min/extends-fire-resistance-to-120-min: committed diff applies to the before-snapshot");
    assert_eq!(produced.fire_resistance_min, 120.0, "change-fire-resistance-min/extends-fire-resistance-to-120-min: the committed diff must set `fire_resistance_min` to 120.0");
    assert_eq!(produced, expected_after(), "change-fire-resistance-min/extends-fire-resistance-to-120-min: committed diff did not carry before to after");
}
