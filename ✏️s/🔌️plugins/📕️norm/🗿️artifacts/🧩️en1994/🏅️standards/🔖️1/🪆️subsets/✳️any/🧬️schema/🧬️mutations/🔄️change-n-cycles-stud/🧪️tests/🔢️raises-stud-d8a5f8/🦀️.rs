//! 🧪️ `change-n-cycles-stud` fixture — `🔢️raises-stud-cycle-count-to-5000000` (EN 1994 composite).
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
    serde_json::from_str(BEFORE).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: mutation decodes")
}

/// ▶️ `change-n-cycles-stud` carries `n_cycles_stud` from 2000000.0 to 5000000.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: mutation applies to its committed before-snapshot");
    assert_eq!(produced.n_cycles_stud, 5000000.0, "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: `n_cycles_stud` must read 5000000.0 after the mutation");
    assert_eq!(produced.delta_tau_stud_mpa, base.delta_tau_stud_mpa, "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: `delta_tau_stud_mpa` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `n_cycles_stud` (2000000.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: inverse step applies");
    }
    assert_eq!(snapshot.n_cycles_stud, base.n_cycles_stud, "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: inverse must put `n_cycles_stud` back to 2000000.0");
    assert_eq!(snapshot, base, "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: snapshot reparses");
        assert_eq!(reencoded, original, "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: mutation reparses");
    assert_eq!(reencoded, original, "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 2000000.0→5000000.0 edit of `n_cycles_stud` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: changing `n_cycles_stud` away from 2000000.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `nCyclesStud` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().n_cycles_stud, Some(5000000.0), "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: the diff must carry `n_cycles_stud` = 5000000.0");
    assert!(outcome.diff().delta_tau_stud_mpa.is_none(), "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: the diff must leave `delta_tau_stud_mpa` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: committed diff decodes");
    assert_eq!(produced, committed, "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: committed diff decodes");
    assert_eq!(decoded.n_cycles_stud, Some(5000000.0), "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: the committed diff must name `n_cycles_stud` = 5000000.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: committed diff reparses");
    assert_eq!(reencoded, original, "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 5000000.0 `n_cycles_stud` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-n-cycles-stud/raises-stud-cycle-count-to-5000000: committed diff applies to the before-snapshot");
    assert_eq!(produced.n_cycles_stud, 5000000.0, "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: the committed diff must set `n_cycles_stud` to 5000000.0");
    assert_eq!(produced, expected_after(), "change-n-cycles-stud/raises-stud-cycle-count-to-5000000: committed diff did not carry before to after");
}
