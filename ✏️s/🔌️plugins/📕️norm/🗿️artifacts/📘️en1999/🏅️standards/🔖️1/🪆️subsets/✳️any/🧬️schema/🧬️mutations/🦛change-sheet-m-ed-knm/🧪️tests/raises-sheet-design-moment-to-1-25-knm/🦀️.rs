//! 🧪️ `change-sheet-m-ed-knm` fixture — `raises-sheet-design-moment-to-1-25-knm` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: mutation decodes")
}

/// ▶️ `change-sheet-m-ed-knm` carries `sheet_m_ed_knm` from 0.75 to 1.25 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.sheet_m_ed_knm, 1.25, "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: `sheet_m_ed_knm` must read 1.25 after the mutation");
    assert_eq!(produced.shell_t_mm, base.shell_t_mm, "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: `shell_t_mm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `sheet_m_ed_knm` (0.75) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: inverse step applies");
    }
    assert_eq!(snapshot.sheet_m_ed_knm, base.sheet_m_ed_knm, "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: inverse must put `sheet_m_ed_knm` back to 0.75");
    assert_eq!(snapshot, base, "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: snapshot reparses");
        assert_eq!(reencoded, original, "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: mutation reparses");
    assert_eq!(reencoded, original, "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 0.75→1.25 edit of `sheet_m_ed_knm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: changing `sheet_m_ed_knm` away from 0.75 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `sheetMEdKnm` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().sheet_m_ed_knm, Some(1.25), "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: the diff must carry `sheet_m_ed_knm` = 1.25");
    assert!(outcome.diff().shell_t_mm.is_none(), "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: the diff must leave `shell_t_mm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: committed diff decodes");
    assert_eq!(produced, committed, "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: committed diff decodes");
    assert_eq!(decoded.sheet_m_ed_knm, Some(1.25), "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: the committed diff must name `sheet_m_ed_knm` = 1.25");
    let reencoded = serde_json::to_value(&decoded).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: committed diff reparses");
    assert_eq!(reencoded, original, "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 1.25 `sheet_m_ed_knm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: committed diff applies to the before-snapshot");
    assert_eq!(produced.sheet_m_ed_knm, 1.25, "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: the committed diff must set `sheet_m_ed_knm` to 1.25");
    assert_eq!(produced, expected_after(), "change-sheet-m-ed-knm/raises-sheet-design-moment-to-1-25-knm: committed diff did not carry before to after");
}
