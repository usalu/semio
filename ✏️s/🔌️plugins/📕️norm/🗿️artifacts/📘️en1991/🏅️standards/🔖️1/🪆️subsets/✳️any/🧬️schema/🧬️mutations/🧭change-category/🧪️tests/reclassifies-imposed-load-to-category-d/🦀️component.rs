//! 🧪️ `change-category` fixture — `reclassifies-imposed-load-to-category-d` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-category/reclassifies-imposed-load-to-category-d: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-category/reclassifies-imposed-load-to-category-d: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-category/reclassifies-imposed-load-to-category-d: mutation decodes")
}

/// ▶️ `change-category` carries `category` from ImposedCategory::B to ImposedCategory::D and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-category/reclassifies-imposed-load-to-category-d: mutation applies to its committed before-snapshot");
    assert_eq!(produced.category, crate::document::ImposedCategory::D, "change-category/reclassifies-imposed-load-to-category-d: `category` must read ImposedCategory::D after the mutation");
    assert_eq!(produced.annex, base.annex, "change-category/reclassifies-imposed-load-to-category-d: `annex` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-category/reclassifies-imposed-load-to-category-d: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `category` (ImposedCategory::B) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-category/reclassifies-imposed-load-to-category-d: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-category/reclassifies-imposed-load-to-category-d: inverse step applies");
    }
    assert_eq!(snapshot.category, base.category, "change-category/reclassifies-imposed-load-to-category-d: inverse must put `category` back to ImposedCategory::B");
    assert_eq!(snapshot, base, "change-category/reclassifies-imposed-load-to-category-d: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-category/reclassifies-imposed-load-to-category-d: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-category/reclassifies-imposed-load-to-category-d: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-category/reclassifies-imposed-load-to-category-d: snapshot reparses");
        assert_eq!(reencoded, original, "change-category/reclassifies-imposed-load-to-category-d: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-category/reclassifies-imposed-load-to-category-d: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-category/reclassifies-imposed-load-to-category-d: mutation reparses");
    assert_eq!(reencoded, original, "change-category/reclassifies-imposed-load-to-category-d: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean ImposedCategory::B→ImposedCategory::D edit of `category` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-category/reclassifies-imposed-load-to-category-d: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-category/reclassifies-imposed-load-to-category-d: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-category/reclassifies-imposed-load-to-category-d: changing `category` away from ImposedCategory::B must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-category/reclassifies-imposed-load-to-category-d: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `category` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().category, Some(crate::document::ImposedCategory::D), "change-category/reclassifies-imposed-load-to-category-d: the diff must carry `category` = ImposedCategory::D");
    assert!(outcome.diff().annex.is_none(), "change-category/reclassifies-imposed-load-to-category-d: the diff must leave `annex` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-category/reclassifies-imposed-load-to-category-d: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-category/reclassifies-imposed-load-to-category-d: committed diff decodes");
    assert_eq!(produced, committed, "change-category/reclassifies-imposed-load-to-category-d: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-category/reclassifies-imposed-load-to-category-d: committed diff decodes");
    assert_eq!(decoded.category, Some(crate::document::ImposedCategory::D), "change-category/reclassifies-imposed-load-to-category-d: the committed diff must name `category` = ImposedCategory::D");
    let reencoded = serde_json::to_value(&decoded).expect("change-category/reclassifies-imposed-load-to-category-d: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-category/reclassifies-imposed-load-to-category-d: committed diff reparses");
    assert_eq!(reencoded, original, "change-category/reclassifies-imposed-load-to-category-d: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the ImposedCategory::D `category` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-category/reclassifies-imposed-load-to-category-d: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-category/reclassifies-imposed-load-to-category-d: committed diff applies to the before-snapshot");
    assert_eq!(produced.category, crate::document::ImposedCategory::D, "change-category/reclassifies-imposed-load-to-category-d: the committed diff must set `category` to ImposedCategory::D");
    assert_eq!(produced, expected_after(), "change-category/reclassifies-imposed-load-to-category-d: committed diff did not carry before to after");
}
