//! 🧪️ `change-bridge-moment-resistance-knm` fixture — `💪️raises-bridge-moment-resistance-to-4500-knm` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: mutation decodes")
}

/// ▶️ `change-bridge-moment-resistance-knm` carries `bridge_moment_resistance_knm` from 3200.0 to 4500.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.bridge_moment_resistance_knm, 4500.0, "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: `bridge_moment_resistance_knm` must read 4500.0 after the mutation");
    assert_eq!(produced.crane_class, base.crane_class, "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: `crane_class` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `bridge_moment_resistance_knm` (3200.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: inverse step applies");
    }
    assert_eq!(snapshot.bridge_moment_resistance_knm, base.bridge_moment_resistance_knm, "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: inverse must put `bridge_moment_resistance_knm` back to 3200.0");
    assert_eq!(snapshot, base, "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: snapshot reparses");
        assert_eq!(reencoded, original, "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: mutation reparses");
    assert_eq!(reencoded, original, "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 3200.0→4500.0 edit of `bridge_moment_resistance_knm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: changing `bridge_moment_resistance_knm` away from 3200.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `bridgeMomentResistanceKnm` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().bridge_moment_resistance_knm, Some(4500.0), "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: the diff must carry `bridge_moment_resistance_knm` = 4500.0");
    assert!(outcome.diff().crane_class.is_none(), "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: the diff must leave `crane_class` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: committed diff decodes");
    assert_eq!(produced, committed, "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: committed diff decodes");
    assert_eq!(decoded.bridge_moment_resistance_knm, Some(4500.0), "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: the committed diff must name `bridge_moment_resistance_knm` = 4500.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: committed diff reparses");
    assert_eq!(reencoded, original, "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 4500.0 `bridge_moment_resistance_knm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: committed diff applies to the before-snapshot");
    assert_eq!(produced.bridge_moment_resistance_knm, 4500.0, "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: the committed diff must set `bridge_moment_resistance_knm` to 4500.0");
    assert_eq!(produced, expected_after(), "change-bridge-moment-resistance-knm/raises-bridge-moment-resistance-to-4500-knm: committed diff did not carry before to after");
}
