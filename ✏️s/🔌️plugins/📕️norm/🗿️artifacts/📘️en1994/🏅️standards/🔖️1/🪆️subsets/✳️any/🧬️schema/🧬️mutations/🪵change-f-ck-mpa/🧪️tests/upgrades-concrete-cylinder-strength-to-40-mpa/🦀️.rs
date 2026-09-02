//! 🧪️ `change-f-ck-mpa` fixture — `upgrades-concrete-cylinder-strength-to-40-mpa` (EN 1994 composite).
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
    serde_json::from_str(BEFORE).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: mutation decodes")
}

/// ▶️ `change-f-ck-mpa` carries `f_ck_mpa` from 30.0 to 40.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: mutation applies to its committed before-snapshot");
    assert_eq!(produced.f_ck_mpa, 40.0, "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: `f_ck_mpa` must read 40.0 after the mutation");
    assert_eq!(produced.f_u_mpa, base.f_u_mpa, "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: `f_u_mpa` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `f_ck_mpa` (30.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: inverse step applies");
    }
    assert_eq!(snapshot.f_ck_mpa, base.f_ck_mpa, "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: inverse must put `f_ck_mpa` back to 30.0");
    assert_eq!(snapshot, base, "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: snapshot reparses");
        assert_eq!(reencoded, original, "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: mutation reparses");
    assert_eq!(reencoded, original, "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 30.0→40.0 edit of `f_ck_mpa` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: changing `f_ck_mpa` away from 30.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `fCkMpa` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().f_ck_mpa, Some(40.0), "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: the diff must carry `f_ck_mpa` = 40.0");
    assert!(outcome.diff().f_u_mpa.is_none(), "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: the diff must leave `f_u_mpa` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: committed diff decodes");
    assert_eq!(produced, committed, "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: committed diff decodes");
    assert_eq!(decoded.f_ck_mpa, Some(40.0), "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: the committed diff must name `f_ck_mpa` = 40.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: committed diff reparses");
    assert_eq!(reencoded, original, "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 40.0 `f_ck_mpa` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: committed diff applies to the before-snapshot");
    assert_eq!(produced.f_ck_mpa, 40.0, "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: the committed diff must set `f_ck_mpa` to 40.0");
    assert_eq!(produced, expected_after(), "change-f-ck-mpa/upgrades-concrete-cylinder-strength-to-40-mpa: committed diff did not carry before to after");
}
