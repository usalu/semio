//! 🧪️ `change-fy-mpa` fixture — `upgrades-steel-yield-to-460-mpa` (EN 1994 composite).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1994Snapshot {
    serde_json::from_str(BEFORE).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: mutation decodes")
}

/// ▶️ `change-fy-mpa` carries `f_y_mpa` from 355.0 to 460.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: mutation applies to its committed before-snapshot");
    assert_eq!(produced.f_y_mpa, 460.0, "change-fy-mpa/upgrades-steel-yield-to-460-mpa: `f_y_mpa` must read 460.0 after the mutation");
    assert_eq!(produced.n_cycles_stud, base.n_cycles_stud, "change-fy-mpa/upgrades-steel-yield-to-460-mpa: `n_cycles_stud` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-fy-mpa/upgrades-steel-yield-to-460-mpa: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `f_y_mpa` (355.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: inverse step applies");
    }
    assert_eq!(snapshot.f_y_mpa, base.f_y_mpa, "change-fy-mpa/upgrades-steel-yield-to-460-mpa: inverse must put `f_y_mpa` back to 355.0");
    assert_eq!(snapshot, base, "change-fy-mpa/upgrades-steel-yield-to-460-mpa: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: snapshot reparses");
        assert_eq!(reencoded, original, "change-fy-mpa/upgrades-steel-yield-to-460-mpa: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: mutation reparses");
    assert_eq!(reencoded, original, "change-fy-mpa/upgrades-steel-yield-to-460-mpa: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 355.0→460.0 edit of `f_y_mpa` raises no diagnostic.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fy-mpa/upgrades-steel-yield-to-460-mpa: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-fy-mpa/upgrades-steel-yield-to-460-mpa: changing `f_y_mpa` away from 355.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-fy-mpa/upgrades-steel-yield-to-460-mpa: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `fYMpa` set, every other field left null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().f_y_mpa, Some(460.0), "change-fy-mpa/upgrades-steel-yield-to-460-mpa: the diff must carry `f_y_mpa` = 460.0");
    assert!(outcome.diff().n_cycles_stud.is_none(), "change-fy-mpa/upgrades-steel-yield-to-460-mpa: the diff must leave `n_cycles_stud` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: committed diff decodes");
    assert_eq!(produced, committed, "change-fy-mpa/upgrades-steel-yield-to-460-mpa: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: committed diff decodes");
    assert_eq!(decoded.f_y_mpa, Some(460.0), "change-fy-mpa/upgrades-steel-yield-to-460-mpa: the committed diff must name `f_y_mpa` = 460.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: committed diff reparses");
    assert_eq!(reencoded, original, "change-fy-mpa/upgrades-steel-yield-to-460-mpa: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 460.0 `f_y_mpa` edit is complete on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-fy-mpa/upgrades-steel-yield-to-460-mpa: committed diff applies to the before-snapshot");
    assert_eq!(produced.f_y_mpa, 460.0, "change-fy-mpa/upgrades-steel-yield-to-460-mpa: the committed diff must set `f_y_mpa` to 460.0");
    assert_eq!(produced, expected_after(), "change-fy-mpa/upgrades-steel-yield-to-460-mpa: committed diff did not carry before to after");
}
