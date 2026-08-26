//! 🧪️ `change-cd` fixture — `lowers-dynamic-factor-cd-to-0-875` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: mutation decodes")
}

/// ▶️ `change-cd` carries `c_d` from 1.0 to 0.875 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: mutation applies to its committed before-snapshot");
    assert_eq!(produced.c_d, 0.875, "change-cd/lowers-dynamic-factor-cd-to-0-875: `c_d` must read 0.875 after the mutation");
    assert_eq!(produced.area_m2, base.area_m2, "change-cd/lowers-dynamic-factor-cd-to-0-875: `area_m2` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-cd/lowers-dynamic-factor-cd-to-0-875: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `c_d` (1.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: inverse step applies");
    }
    assert_eq!(snapshot.c_d, base.c_d, "change-cd/lowers-dynamic-factor-cd-to-0-875: inverse must put `c_d` back to 1.0");
    assert_eq!(snapshot, base, "change-cd/lowers-dynamic-factor-cd-to-0-875: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: snapshot reparses");
        assert_eq!(reencoded, original, "change-cd/lowers-dynamic-factor-cd-to-0-875: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: mutation reparses");
    assert_eq!(reencoded, original, "change-cd/lowers-dynamic-factor-cd-to-0-875: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 1.0→0.875 edit of `c_d` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-cd/lowers-dynamic-factor-cd-to-0-875: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-cd/lowers-dynamic-factor-cd-to-0-875: changing `c_d` away from 1.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-cd/lowers-dynamic-factor-cd-to-0-875: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `cD` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().c_d, Some(0.875), "change-cd/lowers-dynamic-factor-cd-to-0-875: the diff must carry `c_d` = 0.875");
    assert!(outcome.diff().area_m2.is_none(), "change-cd/lowers-dynamic-factor-cd-to-0-875: the diff must leave `area_m2` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: committed diff decodes");
    assert_eq!(produced, committed, "change-cd/lowers-dynamic-factor-cd-to-0-875: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: committed diff decodes");
    assert_eq!(decoded.c_d, Some(0.875), "change-cd/lowers-dynamic-factor-cd-to-0-875: the committed diff must name `c_d` = 0.875");
    let reencoded = serde_json::to_value(&decoded).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: committed diff reparses");
    assert_eq!(reencoded, original, "change-cd/lowers-dynamic-factor-cd-to-0-875: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 0.875 `c_d` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-cd/lowers-dynamic-factor-cd-to-0-875: committed diff applies to the before-snapshot");
    assert_eq!(produced.c_d, 0.875, "change-cd/lowers-dynamic-factor-cd-to-0-875: the committed diff must set `c_d` to 0.875");
    assert_eq!(produced, expected_after(), "change-cd/lowers-dynamic-factor-cd-to-0-875: committed diff did not carry before to after");
}
