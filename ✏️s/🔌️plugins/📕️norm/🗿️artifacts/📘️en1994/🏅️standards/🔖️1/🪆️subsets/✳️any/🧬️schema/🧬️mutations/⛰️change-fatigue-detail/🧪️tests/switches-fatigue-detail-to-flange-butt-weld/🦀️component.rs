//! 🧪️ `change-fatigue-detail` fixture — `switches-fatigue-detail-to-flange-butt-weld` (EN 1994 composite).
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
    serde_json::from_str(BEFORE).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: mutation decodes")
}

/// ▶️ `change-fatigue-detail` carries `fatigue_detail` from stud_welded to flange_butt_weld and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: mutation applies to its committed before-snapshot");
    assert_eq!(produced.fatigue_detail, "flange_butt_weld", "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: `fatigue_detail` must read flange_butt_weld after the mutation");
    assert_eq!(produced.d_mm, base.d_mm, "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: `d_mm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `fatigue_detail` (stud_welded) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: inverse step applies");
    }
    assert_eq!(snapshot.fatigue_detail, base.fatigue_detail, "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: inverse must put `fatigue_detail` back to stud_welded");
    assert_eq!(snapshot, base, "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: snapshot reparses");
        assert_eq!(reencoded, original, "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: mutation reparses");
    assert_eq!(reencoded, original, "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean stud_welded→flange_butt_weld edit of `fatigue_detail` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: changing `fatigue_detail` away from stud_welded must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `fatigueDetail` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().fatigue_detail.as_deref(), Some("flange_butt_weld"), "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: the diff must carry `fatigue_detail` = flange_butt_weld");
    assert!(outcome.diff().d_mm.is_none(), "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: the diff must leave `d_mm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: committed diff decodes");
    assert_eq!(produced, committed, "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: committed diff decodes");
    assert_eq!(decoded.fatigue_detail.as_deref(), Some("flange_butt_weld"), "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: the committed diff must name `fatigue_detail` = flange_butt_weld");
    let reencoded = serde_json::to_value(&decoded).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: committed diff reparses");
    assert_eq!(reencoded, original, "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the flange_butt_weld `fatigue_detail` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: committed diff applies to the before-snapshot");
    assert_eq!(produced.fatigue_detail, "flange_butt_weld", "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: the committed diff must set `fatigue_detail` to flange_butt_weld");
    assert_eq!(produced, expected_after(), "change-fatigue-detail/switches-fatigue-detail-to-flange-butt-weld: committed diff did not carry before to after");
}
