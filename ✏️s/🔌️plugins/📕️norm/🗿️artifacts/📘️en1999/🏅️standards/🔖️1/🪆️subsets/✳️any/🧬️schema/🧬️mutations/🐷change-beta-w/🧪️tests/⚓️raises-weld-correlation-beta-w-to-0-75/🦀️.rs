//! 🧪️ `change-beta-w` fixture — `⚓️raises-weld-correlation-beta-w-to-0-75` (EN 1999 aluminium).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1999::{En1999Diff, En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1999Snapshot {
    serde_json::from_str(BEFORE).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: mutation decodes")
}

/// ▶️ `change-beta-w` carries `beta_w` from 0.625 to 0.75 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: mutation applies to its committed before-snapshot");
    assert_eq!(produced.beta_w, 0.75, "change-beta-w/raises-weld-correlation-beta-w-to-0-75: `beta_w` must read 0.75 after the mutation");
    assert_eq!(produced.sheet_b_mm, base.sheet_b_mm, "change-beta-w/raises-weld-correlation-beta-w-to-0-75: `sheet_b_mm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-beta-w/raises-weld-correlation-beta-w-to-0-75: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `beta_w` (0.625) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: inverse step applies");
    }
    assert_eq!(snapshot.beta_w, base.beta_w, "change-beta-w/raises-weld-correlation-beta-w-to-0-75: inverse must put `beta_w` back to 0.625");
    assert_eq!(snapshot, base, "change-beta-w/raises-weld-correlation-beta-w-to-0-75: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: snapshot reparses");
        assert_eq!(reencoded, original, "change-beta-w/raises-weld-correlation-beta-w-to-0-75: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: mutation reparses");
    assert_eq!(reencoded, original, "change-beta-w/raises-weld-correlation-beta-w-to-0-75: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 0.625→0.75 edit of `beta_w` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-beta-w/raises-weld-correlation-beta-w-to-0-75: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-beta-w/raises-weld-correlation-beta-w-to-0-75: changing `beta_w` away from 0.625 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-beta-w/raises-weld-correlation-beta-w-to-0-75: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `betaW` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().beta_w, Some(0.75), "change-beta-w/raises-weld-correlation-beta-w-to-0-75: the diff must carry `beta_w` = 0.75");
    assert!(outcome.diff().sheet_b_mm.is_none(), "change-beta-w/raises-weld-correlation-beta-w-to-0-75: the diff must leave `sheet_b_mm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: committed diff decodes");
    assert_eq!(produced, committed, "change-beta-w/raises-weld-correlation-beta-w-to-0-75: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: committed diff decodes");
    assert_eq!(decoded.beta_w, Some(0.75), "change-beta-w/raises-weld-correlation-beta-w-to-0-75: the committed diff must name `beta_w` = 0.75");
    let reencoded = serde_json::to_value(&decoded).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: committed diff reparses");
    assert_eq!(reencoded, original, "change-beta-w/raises-weld-correlation-beta-w-to-0-75: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 0.75 `beta_w` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-beta-w/raises-weld-correlation-beta-w-to-0-75: committed diff applies to the before-snapshot");
    assert_eq!(produced.beta_w, 0.75, "change-beta-w/raises-weld-correlation-beta-w-to-0-75: the committed diff must set `beta_w` to 0.75");
    assert_eq!(produced, expected_after(), "change-beta-w/raises-weld-correlation-beta-w-to-0-75: committed diff did not carry before to after");
}
