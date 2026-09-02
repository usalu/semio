//! 🧪️ `change-span-m` fixture — `lengthens-span-to-12-m` (EN 1994 composite).
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
    serde_json::from_str(BEFORE).expect("change-span-m/lengthens-span-to-12-m: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-span-m/lengthens-span-to-12-m: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-span-m/lengthens-span-to-12-m: mutation decodes")
}

/// ▶️ `change-span-m` carries `span_m` from 9.0 to 12.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-span-m/lengthens-span-to-12-m: mutation applies to its committed before-snapshot");
    assert_eq!(produced.span_m, 12.0, "change-span-m/lengthens-span-to-12-m: `span_m` must read 12.0 after the mutation");
    assert_eq!(produced.f_y_mpa, base.f_y_mpa, "change-span-m/lengthens-span-to-12-m: `f_y_mpa` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-span-m/lengthens-span-to-12-m: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `span_m` (9.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-span-m/lengthens-span-to-12-m: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-span-m/lengthens-span-to-12-m: inverse step applies");
    }
    assert_eq!(snapshot.span_m, base.span_m, "change-span-m/lengthens-span-to-12-m: inverse must put `span_m` back to 9.0");
    assert_eq!(snapshot, base, "change-span-m/lengthens-span-to-12-m: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-span-m/lengthens-span-to-12-m: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-span-m/lengthens-span-to-12-m: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-span-m/lengthens-span-to-12-m: snapshot reparses");
        assert_eq!(reencoded, original, "change-span-m/lengthens-span-to-12-m: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-span-m/lengthens-span-to-12-m: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-span-m/lengthens-span-to-12-m: mutation reparses");
    assert_eq!(reencoded, original, "change-span-m/lengthens-span-to-12-m: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 9.0→12.0 edit of `span_m` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-span-m/lengthens-span-to-12-m: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-span-m/lengthens-span-to-12-m: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-span-m/lengthens-span-to-12-m: changing `span_m` away from 9.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-span-m/lengthens-span-to-12-m: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `spanM` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().span_m, Some(12.0), "change-span-m/lengthens-span-to-12-m: the diff must carry `span_m` = 12.0");
    assert!(outcome.diff().f_y_mpa.is_none(), "change-span-m/lengthens-span-to-12-m: the diff must leave `f_y_mpa` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-span-m/lengthens-span-to-12-m: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-span-m/lengthens-span-to-12-m: committed diff decodes");
    assert_eq!(produced, committed, "change-span-m/lengthens-span-to-12-m: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-span-m/lengthens-span-to-12-m: committed diff decodes");
    assert_eq!(decoded.span_m, Some(12.0), "change-span-m/lengthens-span-to-12-m: the committed diff must name `span_m` = 12.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-span-m/lengthens-span-to-12-m: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-span-m/lengthens-span-to-12-m: committed diff reparses");
    assert_eq!(reencoded, original, "change-span-m/lengthens-span-to-12-m: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 12.0 `span_m` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-span-m/lengthens-span-to-12-m: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-span-m/lengthens-span-to-12-m: committed diff applies to the before-snapshot");
    assert_eq!(produced.span_m, 12.0, "change-span-m/lengthens-span-to-12-m: the committed diff must set `span_m` to 12.0");
    assert_eq!(produced, expected_after(), "change-span-m/lengthens-span-to-12-m: committed diff did not carry before to after");
}
