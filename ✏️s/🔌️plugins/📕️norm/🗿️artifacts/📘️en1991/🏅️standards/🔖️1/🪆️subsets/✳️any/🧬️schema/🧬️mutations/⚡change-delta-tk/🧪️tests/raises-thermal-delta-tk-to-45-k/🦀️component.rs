//! 🧪️ `change-delta-tk` fixture — `raises-thermal-delta-tk-to-45-k` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: mutation decodes")
}

/// ▶️ `change-delta-tk` carries `delta_t_k` from 30.0 to 45.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: mutation applies to its committed before-snapshot");
    assert_eq!(produced.delta_t_k, 45.0, "change-delta-tk/raises-thermal-delta-tk-to-45-k: `delta_t_k` must read 45.0 after the mutation");
    assert_eq!(produced.construction_activity, base.construction_activity, "change-delta-tk/raises-thermal-delta-tk-to-45-k: `construction_activity` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-delta-tk/raises-thermal-delta-tk-to-45-k: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `delta_t_k` (30.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: inverse step applies");
    }
    assert_eq!(snapshot.delta_t_k, base.delta_t_k, "change-delta-tk/raises-thermal-delta-tk-to-45-k: inverse must put `delta_t_k` back to 30.0");
    assert_eq!(snapshot, base, "change-delta-tk/raises-thermal-delta-tk-to-45-k: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: snapshot reparses");
        assert_eq!(reencoded, original, "change-delta-tk/raises-thermal-delta-tk-to-45-k: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: mutation reparses");
    assert_eq!(reencoded, original, "change-delta-tk/raises-thermal-delta-tk-to-45-k: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 30.0→45.0 edit of `delta_t_k` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-delta-tk/raises-thermal-delta-tk-to-45-k: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-delta-tk/raises-thermal-delta-tk-to-45-k: changing `delta_t_k` away from 30.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-delta-tk/raises-thermal-delta-tk-to-45-k: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `deltaTK` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().delta_t_k, Some(45.0), "change-delta-tk/raises-thermal-delta-tk-to-45-k: the diff must carry `delta_t_k` = 45.0");
    assert!(outcome.diff().construction_activity.is_none(), "change-delta-tk/raises-thermal-delta-tk-to-45-k: the diff must leave `construction_activity` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: committed diff decodes");
    assert_eq!(produced, committed, "change-delta-tk/raises-thermal-delta-tk-to-45-k: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: committed diff decodes");
    assert_eq!(decoded.delta_t_k, Some(45.0), "change-delta-tk/raises-thermal-delta-tk-to-45-k: the committed diff must name `delta_t_k` = 45.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: committed diff reparses");
    assert_eq!(reencoded, original, "change-delta-tk/raises-thermal-delta-tk-to-45-k: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 45.0 `delta_t_k` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-delta-tk/raises-thermal-delta-tk-to-45-k: committed diff applies to the before-snapshot");
    assert_eq!(produced.delta_t_k, 45.0, "change-delta-tk/raises-thermal-delta-tk-to-45-k: the committed diff must set `delta_t_k` to 45.0");
    assert_eq!(produced, expected_after(), "change-delta-tk/raises-thermal-delta-tk-to-45-k: committed diff did not carry before to after");
}
