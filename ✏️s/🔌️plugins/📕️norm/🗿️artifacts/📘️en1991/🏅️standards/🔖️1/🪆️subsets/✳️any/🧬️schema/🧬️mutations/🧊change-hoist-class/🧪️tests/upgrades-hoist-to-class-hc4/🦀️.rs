//! 🧪️ `change-hoist-class` fixture — `upgrades-hoist-to-class-hc4` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-hoist-class/upgrades-hoist-to-class-hc4: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-hoist-class/upgrades-hoist-to-class-hc4: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-hoist-class/upgrades-hoist-to-class-hc4: mutation decodes")
}

/// ▶️ `change-hoist-class` carries `hoist_class` from HC2 to HC4 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-hoist-class/upgrades-hoist-to-class-hc4: mutation applies to its committed before-snapshot");
    assert_eq!(produced.hoist_class, "HC4", "change-hoist-class/upgrades-hoist-to-class-hc4: `hoist_class` must read HC4 after the mutation");
    assert_eq!(produced.hoisting_speed_m_s, base.hoisting_speed_m_s, "change-hoist-class/upgrades-hoist-to-class-hc4: `hoisting_speed_m_s` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-hoist-class/upgrades-hoist-to-class-hc4: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `hoist_class` (HC2) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-hoist-class/upgrades-hoist-to-class-hc4: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-hoist-class/upgrades-hoist-to-class-hc4: inverse step applies");
    }
    assert_eq!(snapshot.hoist_class, base.hoist_class, "change-hoist-class/upgrades-hoist-to-class-hc4: inverse must put `hoist_class` back to HC2");
    assert_eq!(snapshot, base, "change-hoist-class/upgrades-hoist-to-class-hc4: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-hoist-class/upgrades-hoist-to-class-hc4: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-hoist-class/upgrades-hoist-to-class-hc4: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-hoist-class/upgrades-hoist-to-class-hc4: snapshot reparses");
        assert_eq!(reencoded, original, "change-hoist-class/upgrades-hoist-to-class-hc4: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-hoist-class/upgrades-hoist-to-class-hc4: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-hoist-class/upgrades-hoist-to-class-hc4: mutation reparses");
    assert_eq!(reencoded, original, "change-hoist-class/upgrades-hoist-to-class-hc4: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean HC2→HC4 edit of `hoist_class` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-hoist-class/upgrades-hoist-to-class-hc4: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-hoist-class/upgrades-hoist-to-class-hc4: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-hoist-class/upgrades-hoist-to-class-hc4: changing `hoist_class` away from HC2 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-hoist-class/upgrades-hoist-to-class-hc4: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `hoistClass` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().hoist_class.as_deref(), Some("HC4"), "change-hoist-class/upgrades-hoist-to-class-hc4: the diff must carry `hoist_class` = HC4");
    assert!(outcome.diff().hoisting_speed_m_s.is_none(), "change-hoist-class/upgrades-hoist-to-class-hc4: the diff must leave `hoisting_speed_m_s` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-hoist-class/upgrades-hoist-to-class-hc4: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-hoist-class/upgrades-hoist-to-class-hc4: committed diff decodes");
    assert_eq!(produced, committed, "change-hoist-class/upgrades-hoist-to-class-hc4: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-hoist-class/upgrades-hoist-to-class-hc4: committed diff decodes");
    assert_eq!(decoded.hoist_class.as_deref(), Some("HC4"), "change-hoist-class/upgrades-hoist-to-class-hc4: the committed diff must name `hoist_class` = HC4");
    let reencoded = serde_json::to_value(&decoded).expect("change-hoist-class/upgrades-hoist-to-class-hc4: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-hoist-class/upgrades-hoist-to-class-hc4: committed diff reparses");
    assert_eq!(reencoded, original, "change-hoist-class/upgrades-hoist-to-class-hc4: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the HC4 `hoist_class` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-hoist-class/upgrades-hoist-to-class-hc4: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-hoist-class/upgrades-hoist-to-class-hc4: committed diff applies to the before-snapshot");
    assert_eq!(produced.hoist_class, "HC4", "change-hoist-class/upgrades-hoist-to-class-hc4: the committed diff must set `hoist_class` to HC4");
    assert_eq!(produced, expected_after(), "change-hoist-class/upgrades-hoist-to-class-hc4: committed diff did not carry before to after");
}
