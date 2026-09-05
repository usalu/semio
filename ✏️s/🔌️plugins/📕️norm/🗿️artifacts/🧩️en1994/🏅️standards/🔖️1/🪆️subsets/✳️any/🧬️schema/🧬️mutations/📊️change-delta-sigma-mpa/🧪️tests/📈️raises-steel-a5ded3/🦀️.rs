//! 🧪️ `change-delta-sigma-mpa` fixture — `📈️raises-steel-stress-range-to-96-mpa` (EN 1994 composite).
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
    serde_json::from_str(BEFORE).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: mutation decodes")
}

/// ▶️ `change-delta-sigma-mpa` carries `delta_sigma_mpa` from 72.0 to 96.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: mutation applies to its committed before-snapshot");
    assert_eq!(produced.delta_sigma_mpa, 96.0, "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: `delta_sigma_mpa` must read 96.0 after the mutation");
    assert_eq!(produced.fatigue_detail, base.fatigue_detail, "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: `fatigue_detail` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `delta_sigma_mpa` (72.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: inverse step applies");
    }
    assert_eq!(snapshot.delta_sigma_mpa, base.delta_sigma_mpa, "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: inverse must put `delta_sigma_mpa` back to 72.0");
    assert_eq!(snapshot, base, "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: snapshot reparses");
        assert_eq!(reencoded, original, "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: mutation reparses");
    assert_eq!(reencoded, original, "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 72.0→96.0 edit of `delta_sigma_mpa` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: changing `delta_sigma_mpa` away from 72.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `deltaSigmaMpa` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().delta_sigma_mpa, Some(96.0), "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: the diff must carry `delta_sigma_mpa` = 96.0");
    assert!(outcome.diff().fatigue_detail.is_none(), "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: the diff must leave `fatigue_detail` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: committed diff decodes");
    assert_eq!(produced, committed, "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: committed diff decodes");
    assert_eq!(decoded.delta_sigma_mpa, Some(96.0), "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: the committed diff must name `delta_sigma_mpa` = 96.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: committed diff reparses");
    assert_eq!(reencoded, original, "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 96.0 `delta_sigma_mpa` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: committed diff applies to the before-snapshot");
    assert_eq!(produced.delta_sigma_mpa, 96.0, "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: the committed diff must set `delta_sigma_mpa` to 96.0");
    assert_eq!(produced, expected_after(), "change-delta-sigma-mpa/raises-steel-stress-range-to-96-mpa: committed diff did not carry before to after");
}
