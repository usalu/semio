//! 🧪️ `change-bridge-lane` fixture — `widens-carriageway-to-3-notional-lanes` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: mutation decodes")
}

/// ▶️ `change-bridge-lane` carries `bridge_lane` from 2 to 3 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: mutation applies to its committed before-snapshot");
    assert_eq!(produced.bridge_lane, 3, "change-bridge-lane/widens-carriageway-to-3-notional-lanes: `bridge_lane` must read 3 after the mutation");
    assert_eq!(produced.bridge_span_m, base.bridge_span_m, "change-bridge-lane/widens-carriageway-to-3-notional-lanes: `bridge_span_m` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-bridge-lane/widens-carriageway-to-3-notional-lanes: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `bridge_lane` (2) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: inverse step applies");
    }
    assert_eq!(snapshot.bridge_lane, base.bridge_lane, "change-bridge-lane/widens-carriageway-to-3-notional-lanes: inverse must put `bridge_lane` back to 2");
    assert_eq!(snapshot, base, "change-bridge-lane/widens-carriageway-to-3-notional-lanes: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: snapshot reparses");
        assert_eq!(reencoded, original, "change-bridge-lane/widens-carriageway-to-3-notional-lanes: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: mutation reparses");
    assert_eq!(reencoded, original, "change-bridge-lane/widens-carriageway-to-3-notional-lanes: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 2→3 edit of `bridge_lane` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-bridge-lane/widens-carriageway-to-3-notional-lanes: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-bridge-lane/widens-carriageway-to-3-notional-lanes: changing `bridge_lane` away from 2 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-bridge-lane/widens-carriageway-to-3-notional-lanes: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `bridgeLane` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().bridge_lane, Some(3), "change-bridge-lane/widens-carriageway-to-3-notional-lanes: the diff must carry `bridge_lane` = 3");
    assert!(outcome.diff().bridge_span_m.is_none(), "change-bridge-lane/widens-carriageway-to-3-notional-lanes: the diff must leave `bridge_span_m` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: committed diff decodes");
    assert_eq!(produced, committed, "change-bridge-lane/widens-carriageway-to-3-notional-lanes: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: committed diff decodes");
    assert_eq!(decoded.bridge_lane, Some(3), "change-bridge-lane/widens-carriageway-to-3-notional-lanes: the committed diff must name `bridge_lane` = 3");
    let reencoded = serde_json::to_value(&decoded).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: committed diff reparses");
    assert_eq!(reencoded, original, "change-bridge-lane/widens-carriageway-to-3-notional-lanes: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 3 `bridge_lane` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-bridge-lane/widens-carriageway-to-3-notional-lanes: committed diff applies to the before-snapshot");
    assert_eq!(produced.bridge_lane, 3, "change-bridge-lane/widens-carriageway-to-3-notional-lanes: the committed diff must set `bridge_lane` to 3");
    assert_eq!(produced, expected_after(), "change-bridge-lane/widens-carriageway-to-3-notional-lanes: committed diff did not carry before to after");
}
