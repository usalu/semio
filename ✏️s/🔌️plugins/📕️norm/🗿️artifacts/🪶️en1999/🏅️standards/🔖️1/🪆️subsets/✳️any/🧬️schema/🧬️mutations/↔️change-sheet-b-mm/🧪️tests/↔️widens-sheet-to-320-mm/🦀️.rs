//! 🧪️ `change-sheet-b-mm` fixture — `↔️widens-sheet-to-320-mm` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-sheet-b-mm/widens-sheet-to-320-mm: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-sheet-b-mm/widens-sheet-to-320-mm: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-sheet-b-mm/widens-sheet-to-320-mm: mutation decodes")
}

/// ▶️ `change-sheet-b-mm` carries `sheet_b_mm` from 240.0 to 320.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-sheet-b-mm/widens-sheet-to-320-mm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.sheet_b_mm, 320.0, "change-sheet-b-mm/widens-sheet-to-320-mm: `sheet_b_mm` must read 320.0 after the mutation");
    assert_eq!(produced.sheet_t_mm, base.sheet_t_mm, "change-sheet-b-mm/widens-sheet-to-320-mm: `sheet_t_mm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-sheet-b-mm/widens-sheet-to-320-mm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `sheet_b_mm` (240.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-sheet-b-mm/widens-sheet-to-320-mm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-sheet-b-mm/widens-sheet-to-320-mm: inverse step applies");
    }
    assert_eq!(snapshot.sheet_b_mm, base.sheet_b_mm, "change-sheet-b-mm/widens-sheet-to-320-mm: inverse must put `sheet_b_mm` back to 240.0");
    assert_eq!(snapshot, base, "change-sheet-b-mm/widens-sheet-to-320-mm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-sheet-b-mm/widens-sheet-to-320-mm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-sheet-b-mm/widens-sheet-to-320-mm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-sheet-b-mm/widens-sheet-to-320-mm: snapshot reparses");
        assert_eq!(reencoded, original, "change-sheet-b-mm/widens-sheet-to-320-mm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-sheet-b-mm/widens-sheet-to-320-mm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-sheet-b-mm/widens-sheet-to-320-mm: mutation reparses");
    assert_eq!(reencoded, original, "change-sheet-b-mm/widens-sheet-to-320-mm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 240.0→320.0 edit of `sheet_b_mm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-sheet-b-mm/widens-sheet-to-320-mm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-sheet-b-mm/widens-sheet-to-320-mm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-sheet-b-mm/widens-sheet-to-320-mm: changing `sheet_b_mm` away from 240.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-sheet-b-mm/widens-sheet-to-320-mm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `sheetBMm` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().sheet_b_mm, Some(320.0), "change-sheet-b-mm/widens-sheet-to-320-mm: the diff must carry `sheet_b_mm` = 320.0");
    assert!(outcome.diff().sheet_t_mm.is_none(), "change-sheet-b-mm/widens-sheet-to-320-mm: the diff must leave `sheet_t_mm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-sheet-b-mm/widens-sheet-to-320-mm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-sheet-b-mm/widens-sheet-to-320-mm: committed diff decodes");
    assert_eq!(produced, committed, "change-sheet-b-mm/widens-sheet-to-320-mm: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sheet-b-mm/widens-sheet-to-320-mm: committed diff decodes");
    assert_eq!(decoded.sheet_b_mm, Some(320.0), "change-sheet-b-mm/widens-sheet-to-320-mm: the committed diff must name `sheet_b_mm` = 320.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-sheet-b-mm/widens-sheet-to-320-mm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-sheet-b-mm/widens-sheet-to-320-mm: committed diff reparses");
    assert_eq!(reencoded, original, "change-sheet-b-mm/widens-sheet-to-320-mm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 320.0 `sheet_b_mm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sheet-b-mm/widens-sheet-to-320-mm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-sheet-b-mm/widens-sheet-to-320-mm: committed diff applies to the before-snapshot");
    assert_eq!(produced.sheet_b_mm, 320.0, "change-sheet-b-mm/widens-sheet-to-320-mm: the committed diff must set `sheet_b_mm` to 320.0");
    assert_eq!(produced, expected_after(), "change-sheet-b-mm/widens-sheet-to-320-mm: committed diff did not carry before to after");
}
