//! 🧪️ `change-sheet-t-mm` fixture — `thickens-sheet-to-3-5-mm` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: mutation decodes")
}

/// ▶️ `change-sheet-t-mm` carries `sheet_t_mm` from 2.5 to 3.5 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.sheet_t_mm, 3.5, "change-sheet-t-mm/thickens-sheet-to-3-5-mm: `sheet_t_mm` must read 3.5 after the mutation");
    assert_eq!(produced.sheet_k_sigma, base.sheet_k_sigma, "change-sheet-t-mm/thickens-sheet-to-3-5-mm: `sheet_k_sigma` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-sheet-t-mm/thickens-sheet-to-3-5-mm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `sheet_t_mm` (2.5) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: inverse step applies");
    }
    assert_eq!(snapshot.sheet_t_mm, base.sheet_t_mm, "change-sheet-t-mm/thickens-sheet-to-3-5-mm: inverse must put `sheet_t_mm` back to 2.5");
    assert_eq!(snapshot, base, "change-sheet-t-mm/thickens-sheet-to-3-5-mm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: snapshot reparses");
        assert_eq!(reencoded, original, "change-sheet-t-mm/thickens-sheet-to-3-5-mm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: mutation reparses");
    assert_eq!(reencoded, original, "change-sheet-t-mm/thickens-sheet-to-3-5-mm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 2.5→3.5 edit of `sheet_t_mm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-sheet-t-mm/thickens-sheet-to-3-5-mm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-sheet-t-mm/thickens-sheet-to-3-5-mm: changing `sheet_t_mm` away from 2.5 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-sheet-t-mm/thickens-sheet-to-3-5-mm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `sheetTMm` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().sheet_t_mm, Some(3.5), "change-sheet-t-mm/thickens-sheet-to-3-5-mm: the diff must carry `sheet_t_mm` = 3.5");
    assert!(outcome.diff().sheet_k_sigma.is_none(), "change-sheet-t-mm/thickens-sheet-to-3-5-mm: the diff must leave `sheet_k_sigma` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: committed diff decodes");
    assert_eq!(produced, committed, "change-sheet-t-mm/thickens-sheet-to-3-5-mm: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: committed diff decodes");
    assert_eq!(decoded.sheet_t_mm, Some(3.5), "change-sheet-t-mm/thickens-sheet-to-3-5-mm: the committed diff must name `sheet_t_mm` = 3.5");
    let reencoded = serde_json::to_value(&decoded).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: committed diff reparses");
    assert_eq!(reencoded, original, "change-sheet-t-mm/thickens-sheet-to-3-5-mm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 3.5 `sheet_t_mm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-sheet-t-mm/thickens-sheet-to-3-5-mm: committed diff applies to the before-snapshot");
    assert_eq!(produced.sheet_t_mm, 3.5, "change-sheet-t-mm/thickens-sheet-to-3-5-mm: the committed diff must set `sheet_t_mm` to 3.5");
    assert_eq!(produced, expected_after(), "change-sheet-t-mm/thickens-sheet-to-3-5-mm: committed diff did not carry before to after");
}
