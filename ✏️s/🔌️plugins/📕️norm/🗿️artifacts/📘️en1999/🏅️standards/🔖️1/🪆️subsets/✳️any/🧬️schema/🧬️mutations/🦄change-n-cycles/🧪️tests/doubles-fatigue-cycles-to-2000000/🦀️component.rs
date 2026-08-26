//! 🧪️ `change-n-cycles` fixture — `doubles-fatigue-cycles-to-2000000` (EN 1999 aluminium).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1999::{En1999Diff, En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1999Snapshot {
    serde_json::from_str(BEFORE).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: mutation decodes")
}

/// ▶️ `change-n-cycles` carries `n_cycles` from 1000000.0 to 2000000.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: mutation applies to its committed before-snapshot");
    assert_eq!(produced.n_cycles, 2000000.0, "change-n-cycles/doubles-fatigue-cycles-to-2000000: `n_cycles` must read 2000000.0 after the mutation");
    assert_eq!(produced.v_weld_ed_kn, base.v_weld_ed_kn, "change-n-cycles/doubles-fatigue-cycles-to-2000000: `v_weld_ed_kn` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-n-cycles/doubles-fatigue-cycles-to-2000000: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `n_cycles` (1000000.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: inverse step applies");
    }
    assert_eq!(snapshot.n_cycles, base.n_cycles, "change-n-cycles/doubles-fatigue-cycles-to-2000000: inverse must put `n_cycles` back to 1000000.0");
    assert_eq!(snapshot, base, "change-n-cycles/doubles-fatigue-cycles-to-2000000: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: snapshot reparses");
        assert_eq!(reencoded, original, "change-n-cycles/doubles-fatigue-cycles-to-2000000: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: mutation reparses");
    assert_eq!(reencoded, original, "change-n-cycles/doubles-fatigue-cycles-to-2000000: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 1000000.0→2000000.0 edit of `n_cycles` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-n-cycles/doubles-fatigue-cycles-to-2000000: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-n-cycles/doubles-fatigue-cycles-to-2000000: changing `n_cycles` away from 1000000.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-n-cycles/doubles-fatigue-cycles-to-2000000: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `nCycles` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().n_cycles, Some(2000000.0), "change-n-cycles/doubles-fatigue-cycles-to-2000000: the diff must carry `n_cycles` = 2000000.0");
    assert!(outcome.diff().v_weld_ed_kn.is_none(), "change-n-cycles/doubles-fatigue-cycles-to-2000000: the diff must leave `v_weld_ed_kn` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: committed diff decodes");
    assert_eq!(produced, committed, "change-n-cycles/doubles-fatigue-cycles-to-2000000: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: committed diff decodes");
    assert_eq!(decoded.n_cycles, Some(2000000.0), "change-n-cycles/doubles-fatigue-cycles-to-2000000: the committed diff must name `n_cycles` = 2000000.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: committed diff reparses");
    assert_eq!(reencoded, original, "change-n-cycles/doubles-fatigue-cycles-to-2000000: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 2000000.0 `n_cycles` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-n-cycles/doubles-fatigue-cycles-to-2000000: committed diff applies to the before-snapshot");
    assert_eq!(produced.n_cycles, 2000000.0, "change-n-cycles/doubles-fatigue-cycles-to-2000000: the committed diff must set `n_cycles` to 2000000.0");
    assert_eq!(produced, expected_after(), "change-n-cycles/doubles-fatigue-cycles-to-2000000: committed diff did not carry before to after");
}
