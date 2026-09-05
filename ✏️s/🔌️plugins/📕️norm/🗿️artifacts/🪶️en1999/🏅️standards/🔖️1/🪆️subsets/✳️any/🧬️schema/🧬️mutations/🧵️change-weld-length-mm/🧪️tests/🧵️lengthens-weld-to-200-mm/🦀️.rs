//! 🧪️ `change-weld-length-mm` fixture — `🧵️lengthens-weld-to-200-mm` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-weld-length-mm/lengthens-weld-to-200-mm: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-weld-length-mm/lengthens-weld-to-200-mm: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-weld-length-mm/lengthens-weld-to-200-mm: mutation decodes")
}

/// ▶️ `change-weld-length-mm` carries `weld_length_mm` from 150.0 to 200.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-weld-length-mm/lengthens-weld-to-200-mm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.weld_length_mm, 200.0, "change-weld-length-mm/lengthens-weld-to-200-mm: `weld_length_mm` must read 200.0 after the mutation");
    assert_eq!(produced.beta_w, base.beta_w, "change-weld-length-mm/lengthens-weld-to-200-mm: `beta_w` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-weld-length-mm/lengthens-weld-to-200-mm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `weld_length_mm` (150.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-weld-length-mm/lengthens-weld-to-200-mm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-weld-length-mm/lengthens-weld-to-200-mm: inverse step applies");
    }
    assert_eq!(snapshot.weld_length_mm, base.weld_length_mm, "change-weld-length-mm/lengthens-weld-to-200-mm: inverse must put `weld_length_mm` back to 150.0");
    assert_eq!(snapshot, base, "change-weld-length-mm/lengthens-weld-to-200-mm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-weld-length-mm/lengthens-weld-to-200-mm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-weld-length-mm/lengthens-weld-to-200-mm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-weld-length-mm/lengthens-weld-to-200-mm: snapshot reparses");
        assert_eq!(reencoded, original, "change-weld-length-mm/lengthens-weld-to-200-mm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-weld-length-mm/lengthens-weld-to-200-mm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-weld-length-mm/lengthens-weld-to-200-mm: mutation reparses");
    assert_eq!(reencoded, original, "change-weld-length-mm/lengthens-weld-to-200-mm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 150.0→200.0 edit of `weld_length_mm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-weld-length-mm/lengthens-weld-to-200-mm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-weld-length-mm/lengthens-weld-to-200-mm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-weld-length-mm/lengthens-weld-to-200-mm: changing `weld_length_mm` away from 150.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-weld-length-mm/lengthens-weld-to-200-mm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `weldLengthMm` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().weld_length_mm, Some(200.0), "change-weld-length-mm/lengthens-weld-to-200-mm: the diff must carry `weld_length_mm` = 200.0");
    assert!(outcome.diff().beta_w.is_none(), "change-weld-length-mm/lengthens-weld-to-200-mm: the diff must leave `beta_w` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-weld-length-mm/lengthens-weld-to-200-mm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-weld-length-mm/lengthens-weld-to-200-mm: committed diff decodes");
    assert_eq!(produced, committed, "change-weld-length-mm/lengthens-weld-to-200-mm: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-weld-length-mm/lengthens-weld-to-200-mm: committed diff decodes");
    assert_eq!(decoded.weld_length_mm, Some(200.0), "change-weld-length-mm/lengthens-weld-to-200-mm: the committed diff must name `weld_length_mm` = 200.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-weld-length-mm/lengthens-weld-to-200-mm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-weld-length-mm/lengthens-weld-to-200-mm: committed diff reparses");
    assert_eq!(reencoded, original, "change-weld-length-mm/lengthens-weld-to-200-mm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 200.0 `weld_length_mm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-weld-length-mm/lengthens-weld-to-200-mm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-weld-length-mm/lengthens-weld-to-200-mm: committed diff applies to the before-snapshot");
    assert_eq!(produced.weld_length_mm, 200.0, "change-weld-length-mm/lengthens-weld-to-200-mm: the committed diff must set `weld_length_mm` to 200.0");
    assert_eq!(produced, expected_after(), "change-weld-length-mm/lengthens-weld-to-200-mm: committed diff did not carry before to after");
}
