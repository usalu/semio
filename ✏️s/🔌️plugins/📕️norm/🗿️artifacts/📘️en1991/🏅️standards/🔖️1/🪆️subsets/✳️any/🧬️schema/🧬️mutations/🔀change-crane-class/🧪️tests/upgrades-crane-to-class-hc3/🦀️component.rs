//! 🧪️ `change-crane-class` fixture — `upgrades-crane-to-class-hc3` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-crane-class/upgrades-crane-to-class-hc3: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-crane-class/upgrades-crane-to-class-hc3: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-crane-class/upgrades-crane-to-class-hc3: mutation decodes")
}

/// ▶️ `change-crane-class` carries `crane_class` from HC2 to HC3 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-crane-class/upgrades-crane-to-class-hc3: mutation applies to its committed before-snapshot");
    assert_eq!(produced.crane_class, "HC3", "change-crane-class/upgrades-crane-to-class-hc3: `crane_class` must read HC3 after the mutation");
    assert_eq!(produced.hoist_class, base.hoist_class, "change-crane-class/upgrades-crane-to-class-hc3: `hoist_class` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-crane-class/upgrades-crane-to-class-hc3: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `crane_class` (HC2) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-crane-class/upgrades-crane-to-class-hc3: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-crane-class/upgrades-crane-to-class-hc3: inverse step applies");
    }
    assert_eq!(snapshot.crane_class, base.crane_class, "change-crane-class/upgrades-crane-to-class-hc3: inverse must put `crane_class` back to HC2");
    assert_eq!(snapshot, base, "change-crane-class/upgrades-crane-to-class-hc3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-crane-class/upgrades-crane-to-class-hc3: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-crane-class/upgrades-crane-to-class-hc3: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-crane-class/upgrades-crane-to-class-hc3: snapshot reparses");
        assert_eq!(reencoded, original, "change-crane-class/upgrades-crane-to-class-hc3: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-crane-class/upgrades-crane-to-class-hc3: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-crane-class/upgrades-crane-to-class-hc3: mutation reparses");
    assert_eq!(reencoded, original, "change-crane-class/upgrades-crane-to-class-hc3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean HC2→HC3 edit of `crane_class` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-crane-class/upgrades-crane-to-class-hc3: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-crane-class/upgrades-crane-to-class-hc3: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-crane-class/upgrades-crane-to-class-hc3: changing `crane_class` away from HC2 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-crane-class/upgrades-crane-to-class-hc3: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `craneClass` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().crane_class.as_deref(), Some("HC3"), "change-crane-class/upgrades-crane-to-class-hc3: the diff must carry `crane_class` = HC3");
    assert!(outcome.diff().hoist_class.is_none(), "change-crane-class/upgrades-crane-to-class-hc3: the diff must leave `hoist_class` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-crane-class/upgrades-crane-to-class-hc3: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-crane-class/upgrades-crane-to-class-hc3: committed diff decodes");
    assert_eq!(produced, committed, "change-crane-class/upgrades-crane-to-class-hc3: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-crane-class/upgrades-crane-to-class-hc3: committed diff decodes");
    assert_eq!(decoded.crane_class.as_deref(), Some("HC3"), "change-crane-class/upgrades-crane-to-class-hc3: the committed diff must name `crane_class` = HC3");
    let reencoded = serde_json::to_value(&decoded).expect("change-crane-class/upgrades-crane-to-class-hc3: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-crane-class/upgrades-crane-to-class-hc3: committed diff reparses");
    assert_eq!(reencoded, original, "change-crane-class/upgrades-crane-to-class-hc3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the HC3 `crane_class` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-crane-class/upgrades-crane-to-class-hc3: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-crane-class/upgrades-crane-to-class-hc3: committed diff applies to the before-snapshot");
    assert_eq!(produced.crane_class, "HC3", "change-crane-class/upgrades-crane-to-class-hc3: the committed diff must set `crane_class` to HC3");
    assert_eq!(produced, expected_after(), "change-crane-class/upgrades-crane-to-class-hc3: committed diff did not carry before to after");
}
