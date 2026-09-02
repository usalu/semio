//! 🧪️ `change-eta` fixture — `raises-shear-connection-degree-to-0-875` (EN 1994 composite).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1994Snapshot {
    serde_json::from_str(BEFORE).expect("change-eta/raises-shear-connection-degree-to-0-875: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-eta/raises-shear-connection-degree-to-0-875: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-eta/raises-shear-connection-degree-to-0-875: mutation decodes")
}

/// ▶️ `change-eta` carries `eta` from 0.75 to 0.875 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-eta/raises-shear-connection-degree-to-0-875: mutation applies to its committed before-snapshot");
    assert_eq!(produced.eta, 0.875, "change-eta/raises-shear-connection-degree-to-0-875: `eta` must read 0.875 after the mutation");
    assert_eq!(produced.v_l_rd, base.v_l_rd, "change-eta/raises-shear-connection-degree-to-0-875: `v_l_rd` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-eta/raises-shear-connection-degree-to-0-875: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `eta` (0.75) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-eta/raises-shear-connection-degree-to-0-875: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-eta/raises-shear-connection-degree-to-0-875: inverse step applies");
    }
    assert_eq!(snapshot.eta, base.eta, "change-eta/raises-shear-connection-degree-to-0-875: inverse must put `eta` back to 0.75");
    assert_eq!(snapshot, base, "change-eta/raises-shear-connection-degree-to-0-875: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-eta/raises-shear-connection-degree-to-0-875: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-eta/raises-shear-connection-degree-to-0-875: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-eta/raises-shear-connection-degree-to-0-875: snapshot reparses");
        assert_eq!(reencoded, original, "change-eta/raises-shear-connection-degree-to-0-875: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-eta/raises-shear-connection-degree-to-0-875: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-eta/raises-shear-connection-degree-to-0-875: mutation reparses");
    assert_eq!(reencoded, original, "change-eta/raises-shear-connection-degree-to-0-875: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 0.75→0.875 edit of `eta` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-eta/raises-shear-connection-degree-to-0-875: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-eta/raises-shear-connection-degree-to-0-875: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-eta/raises-shear-connection-degree-to-0-875: changing `eta` away from 0.75 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-eta/raises-shear-connection-degree-to-0-875: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `eta` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().eta, Some(0.875), "change-eta/raises-shear-connection-degree-to-0-875: the diff must carry `eta` = 0.875");
    assert!(outcome.diff().v_l_rd.is_none(), "change-eta/raises-shear-connection-degree-to-0-875: the diff must leave `v_l_rd` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-eta/raises-shear-connection-degree-to-0-875: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-eta/raises-shear-connection-degree-to-0-875: committed diff decodes");
    assert_eq!(produced, committed, "change-eta/raises-shear-connection-degree-to-0-875: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-eta/raises-shear-connection-degree-to-0-875: committed diff decodes");
    assert_eq!(decoded.eta, Some(0.875), "change-eta/raises-shear-connection-degree-to-0-875: the committed diff must name `eta` = 0.875");
    let reencoded = serde_json::to_value(&decoded).expect("change-eta/raises-shear-connection-degree-to-0-875: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-eta/raises-shear-connection-degree-to-0-875: committed diff reparses");
    assert_eq!(reencoded, original, "change-eta/raises-shear-connection-degree-to-0-875: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 0.875 `eta` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-eta/raises-shear-connection-degree-to-0-875: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-eta/raises-shear-connection-degree-to-0-875: committed diff applies to the before-snapshot");
    assert_eq!(produced.eta, 0.875, "change-eta/raises-shear-connection-degree-to-0-875: the committed diff must set `eta` to 0.875");
    assert_eq!(produced, expected_after(), "change-eta/raises-shear-connection-degree-to-0-875: committed diff did not carry before to after");
}
