//! 🧪️ `change-e-cm-mpa` fixture — `raises-concrete-modulus-to-35000-mpa` (EN 1994 composite).
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
    serde_json::from_str(BEFORE).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: mutation decodes")
}

/// ▶️ `change-e-cm-mpa` carries `e_cm_mpa` from 33000.0 to 35000.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: mutation applies to its committed before-snapshot");
    assert_eq!(produced.e_cm_mpa, 35000.0, "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: `e_cm_mpa` must read 35000.0 after the mutation");
    assert_eq!(produced.v_ed_per_stud_kn, base.v_ed_per_stud_kn, "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: `v_ed_per_stud_kn` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `e_cm_mpa` (33000.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: inverse step applies");
    }
    assert_eq!(snapshot.e_cm_mpa, base.e_cm_mpa, "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: inverse must put `e_cm_mpa` back to 33000.0");
    assert_eq!(snapshot, base, "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: snapshot reparses");
        assert_eq!(reencoded, original, "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: mutation reparses");
    assert_eq!(reencoded, original, "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 33000.0→35000.0 edit of `e_cm_mpa` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: changing `e_cm_mpa` away from 33000.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `eCmMpa` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().e_cm_mpa, Some(35000.0), "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: the diff must carry `e_cm_mpa` = 35000.0");
    assert!(outcome.diff().v_ed_per_stud_kn.is_none(), "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: the diff must leave `v_ed_per_stud_kn` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: committed diff decodes");
    assert_eq!(produced, committed, "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: committed diff decodes");
    assert_eq!(decoded.e_cm_mpa, Some(35000.0), "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: the committed diff must name `e_cm_mpa` = 35000.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: committed diff reparses");
    assert_eq!(reencoded, original, "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 35000.0 `e_cm_mpa` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: committed diff applies to the before-snapshot");
    assert_eq!(produced.e_cm_mpa, 35000.0, "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: the committed diff must set `e_cm_mpa` to 35000.0");
    assert_eq!(produced, expected_after(), "change-e-cm-mpa/raises-concrete-modulus-to-35000-mpa: committed diff did not carry before to after");
}
