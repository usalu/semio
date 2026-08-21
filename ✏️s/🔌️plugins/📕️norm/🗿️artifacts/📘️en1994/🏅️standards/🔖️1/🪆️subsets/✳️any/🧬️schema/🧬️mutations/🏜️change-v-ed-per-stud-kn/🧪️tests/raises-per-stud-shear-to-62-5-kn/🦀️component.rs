//! 🧪️ `change-v-ed-per-stud-kn` fixture — `raises-per-stud-shear-to-62-5-kn` (EN 1994 composite).
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
    serde_json::from_str(BEFORE).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: mutation decodes")
}

/// ▶️ `change-v-ed-per-stud-kn` carries `v_ed_per_stud_kn` from 48.0 to 62.5 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: mutation applies to its committed before-snapshot");
    assert_eq!(produced.v_ed_per_stud_kn, 62.5, "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: `v_ed_per_stud_kn` must read 62.5 after the mutation");
    assert_eq!(produced.span_m, base.span_m, "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: `span_m` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `v_ed_per_stud_kn` (48.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: inverse step applies");
    }
    assert_eq!(snapshot.v_ed_per_stud_kn, base.v_ed_per_stud_kn, "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: inverse must put `v_ed_per_stud_kn` back to 48.0");
    assert_eq!(snapshot, base, "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: snapshot reparses");
        assert_eq!(reencoded, original, "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: mutation reparses");
    assert_eq!(reencoded, original, "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 48.0→62.5 edit of `v_ed_per_stud_kn` raises no diagnostic.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: changing `v_ed_per_stud_kn` away from 48.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `vEdPerStudKn` set, every other field left null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().v_ed_per_stud_kn, Some(62.5), "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: the diff must carry `v_ed_per_stud_kn` = 62.5");
    assert!(outcome.diff().span_m.is_none(), "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: the diff must leave `span_m` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: committed diff decodes");
    assert_eq!(produced, committed, "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: committed diff decodes");
    assert_eq!(decoded.v_ed_per_stud_kn, Some(62.5), "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: the committed diff must name `v_ed_per_stud_kn` = 62.5");
    let reencoded = serde_json::to_value(&decoded).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: committed diff reparses");
    assert_eq!(reencoded, original, "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 62.5 `v_ed_per_stud_kn` edit is complete on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: committed diff applies to the before-snapshot");
    assert_eq!(produced.v_ed_per_stud_kn, 62.5, "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: the committed diff must set `v_ed_per_stud_kn` to 62.5");
    assert_eq!(produced, expected_after(), "change-v-ed-per-stud-kn/raises-per-stud-shear-to-62-5-kn: committed diff did not carry before to after");
}
