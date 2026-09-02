//! 🧪️ `change-sheet-w-el-mm3` fixture — `raises-sheet-section-modulus-to-12800-mm3` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: mutation decodes")
}

/// ▶️ `change-sheet-w-el-mm3` carries `sheet_w_el_mm3` from 9600.0 to 12800.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: mutation applies to its committed before-snapshot");
    assert_eq!(produced.sheet_w_el_mm3, 12800.0, "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: `sheet_w_el_mm3` must read 12800.0 after the mutation");
    assert_eq!(produced.sheet_m_ed_knm, base.sheet_m_ed_knm, "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: `sheet_m_ed_knm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `sheet_w_el_mm3` (9600.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: inverse step applies");
    }
    assert_eq!(snapshot.sheet_w_el_mm3, base.sheet_w_el_mm3, "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: inverse must put `sheet_w_el_mm3` back to 9600.0");
    assert_eq!(snapshot, base, "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: snapshot reparses");
        assert_eq!(reencoded, original, "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: mutation reparses");
    assert_eq!(reencoded, original, "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 9600.0→12800.0 edit of `sheet_w_el_mm3` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: changing `sheet_w_el_mm3` away from 9600.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `sheetWElMm3` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().sheet_w_el_mm3, Some(12800.0), "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: the diff must carry `sheet_w_el_mm3` = 12800.0");
    assert!(outcome.diff().sheet_m_ed_knm.is_none(), "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: the diff must leave `sheet_m_ed_knm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: committed diff decodes");
    assert_eq!(produced, committed, "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: committed diff decodes");
    assert_eq!(decoded.sheet_w_el_mm3, Some(12800.0), "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: the committed diff must name `sheet_w_el_mm3` = 12800.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: committed diff reparses");
    assert_eq!(reencoded, original, "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 12800.0 `sheet_w_el_mm3` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: committed diff applies to the before-snapshot");
    assert_eq!(produced.sheet_w_el_mm3, 12800.0, "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: the committed diff must set `sheet_w_el_mm3` to 12800.0");
    assert_eq!(produced, expected_after(), "change-sheet-w-el-mm3/raises-sheet-section-modulus-to-12800-mm3: committed diff did not carry before to after");
}
