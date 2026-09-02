//! 🧪️ `change-area-m2` fixture — `enlarges-loaded-area-to-360-m2` (EN 1991 actions).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1991Snapshot {
    serde_json::from_str(BEFORE).expect("change-area-m2/enlarges-loaded-area-to-360-m2: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-area-m2/enlarges-loaded-area-to-360-m2: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-area-m2/enlarges-loaded-area-to-360-m2: mutation decodes")
}

/// ▶️ `change-area-m2` carries `area_m2` from 240.0 to 360.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-area-m2/enlarges-loaded-area-to-360-m2: mutation applies to its committed before-snapshot");
    assert_eq!(produced.area_m2, 360.0, "change-area-m2/enlarges-loaded-area-to-360-m2: `area_m2` must read 360.0 after the mutation");
    assert_eq!(produced.category, base.category, "change-area-m2/enlarges-loaded-area-to-360-m2: `category` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-area-m2/enlarges-loaded-area-to-360-m2: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `area_m2` (240.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-area-m2/enlarges-loaded-area-to-360-m2: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-area-m2/enlarges-loaded-area-to-360-m2: inverse step applies");
    }
    assert_eq!(snapshot.area_m2, base.area_m2, "change-area-m2/enlarges-loaded-area-to-360-m2: inverse must put `area_m2` back to 240.0");
    assert_eq!(snapshot, base, "change-area-m2/enlarges-loaded-area-to-360-m2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-area-m2/enlarges-loaded-area-to-360-m2: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-area-m2/enlarges-loaded-area-to-360-m2: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-area-m2/enlarges-loaded-area-to-360-m2: snapshot reparses");
        assert_eq!(reencoded, original, "change-area-m2/enlarges-loaded-area-to-360-m2: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-area-m2/enlarges-loaded-area-to-360-m2: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-area-m2/enlarges-loaded-area-to-360-m2: mutation reparses");
    assert_eq!(reencoded, original, "change-area-m2/enlarges-loaded-area-to-360-m2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 240.0→360.0 edit of `area_m2` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-area-m2/enlarges-loaded-area-to-360-m2: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-area-m2/enlarges-loaded-area-to-360-m2: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-area-m2/enlarges-loaded-area-to-360-m2: changing `area_m2` away from 240.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-area-m2/enlarges-loaded-area-to-360-m2: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `areaM2` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().area_m2, Some(360.0), "change-area-m2/enlarges-loaded-area-to-360-m2: the diff must carry `area_m2` = 360.0");
    assert!(outcome.diff().category.is_none(), "change-area-m2/enlarges-loaded-area-to-360-m2: the diff must leave `category` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-area-m2/enlarges-loaded-area-to-360-m2: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-area-m2/enlarges-loaded-area-to-360-m2: committed diff decodes");
    assert_eq!(produced, committed, "change-area-m2/enlarges-loaded-area-to-360-m2: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-area-m2/enlarges-loaded-area-to-360-m2: committed diff decodes");
    assert_eq!(decoded.area_m2, Some(360.0), "change-area-m2/enlarges-loaded-area-to-360-m2: the committed diff must name `area_m2` = 360.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-area-m2/enlarges-loaded-area-to-360-m2: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-area-m2/enlarges-loaded-area-to-360-m2: committed diff reparses");
    assert_eq!(reencoded, original, "change-area-m2/enlarges-loaded-area-to-360-m2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 360.0 `area_m2` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-area-m2/enlarges-loaded-area-to-360-m2: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-area-m2/enlarges-loaded-area-to-360-m2: committed diff applies to the before-snapshot");
    assert_eq!(produced.area_m2, 360.0, "change-area-m2/enlarges-loaded-area-to-360-m2: the committed diff must set `area_m2` to 360.0");
    assert_eq!(produced, expected_after(), "change-area-m2/enlarges-loaded-area-to-360-m2: committed diff did not carry before to after");
}
