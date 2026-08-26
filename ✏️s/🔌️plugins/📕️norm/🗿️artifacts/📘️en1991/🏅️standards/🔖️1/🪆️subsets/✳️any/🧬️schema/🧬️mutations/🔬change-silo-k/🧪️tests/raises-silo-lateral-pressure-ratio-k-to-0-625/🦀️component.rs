//! 🧪️ `change-silo-k` fixture — `raises-silo-lateral-pressure-ratio-k-to-0-625` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: mutation decodes")
}

/// ▶️ `change-silo-k` carries `silo_k` from 0.5 to 0.625 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: mutation applies to its committed before-snapshot");
    assert_eq!(produced.silo_k, 0.625, "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: `silo_k` must read 0.625 after the mutation");
    assert_eq!(produced.c_s, base.c_s, "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: `c_s` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `silo_k` (0.5) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: inverse step applies");
    }
    assert_eq!(snapshot.silo_k, base.silo_k, "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: inverse must put `silo_k` back to 0.5");
    assert_eq!(snapshot, base, "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: snapshot reparses");
        assert_eq!(reencoded, original, "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: mutation reparses");
    assert_eq!(reencoded, original, "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 0.5→0.625 edit of `silo_k` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: changing `silo_k` away from 0.5 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `siloK` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().silo_k, Some(0.625), "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: the diff must carry `silo_k` = 0.625");
    assert!(outcome.diff().c_s.is_none(), "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: the diff must leave `c_s` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: committed diff decodes");
    assert_eq!(produced, committed, "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: committed diff decodes");
    assert_eq!(decoded.silo_k, Some(0.625), "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: the committed diff must name `silo_k` = 0.625");
    let reencoded = serde_json::to_value(&decoded).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: committed diff reparses");
    assert_eq!(reencoded, original, "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 0.625 `silo_k` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: committed diff applies to the before-snapshot");
    assert_eq!(produced.silo_k, 0.625, "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: the committed diff must set `silo_k` to 0.625");
    assert_eq!(produced, expected_after(), "change-silo-k/raises-silo-lateral-pressure-ratio-k-to-0-625: committed diff did not carry before to after");
}
