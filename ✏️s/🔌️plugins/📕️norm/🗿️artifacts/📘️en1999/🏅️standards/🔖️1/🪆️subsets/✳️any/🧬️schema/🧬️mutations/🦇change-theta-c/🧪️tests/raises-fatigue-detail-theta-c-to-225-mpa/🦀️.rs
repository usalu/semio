//! 🧪️ `change-theta-c` fixture — `raises-fatigue-detail-theta-c-to-225-mpa` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: mutation decodes")
}

/// ▶️ `change-theta-c` carries `theta_c` from 180.0 to 225.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: mutation applies to its committed before-snapshot");
    assert_eq!(produced.theta_c, 225.0, "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: `theta_c` must read 225.0 after the mutation");
    assert_eq!(produced.delta_sigma_ed, base.delta_sigma_ed, "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: `delta_sigma_ed` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `theta_c` (180.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: inverse step applies");
    }
    assert_eq!(snapshot.theta_c, base.theta_c, "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: inverse must put `theta_c` back to 180.0");
    assert_eq!(snapshot, base, "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: snapshot reparses");
        assert_eq!(reencoded, original, "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: mutation reparses");
    assert_eq!(reencoded, original, "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 180.0→225.0 edit of `theta_c` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: changing `theta_c` away from 180.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `thetaC` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().theta_c, Some(225.0), "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: the diff must carry `theta_c` = 225.0");
    assert!(outcome.diff().delta_sigma_ed.is_none(), "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: the diff must leave `delta_sigma_ed` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: committed diff decodes");
    assert_eq!(produced, committed, "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: committed diff decodes");
    assert_eq!(decoded.theta_c, Some(225.0), "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: the committed diff must name `theta_c` = 225.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: committed diff reparses");
    assert_eq!(reencoded, original, "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 225.0 `theta_c` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: committed diff applies to the before-snapshot");
    assert_eq!(produced.theta_c, 225.0, "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: the committed diff must set `theta_c` to 225.0");
    assert_eq!(produced, expected_after(), "change-theta-c/raises-fatigue-detail-theta-c-to-225-mpa: committed diff did not carry before to after");
}
