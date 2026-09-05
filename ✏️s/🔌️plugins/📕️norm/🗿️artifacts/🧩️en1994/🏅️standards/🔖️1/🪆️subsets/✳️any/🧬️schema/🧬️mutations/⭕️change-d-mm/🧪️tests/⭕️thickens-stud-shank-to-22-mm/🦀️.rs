//! 🧪️ `change-d-mm` fixture — `⭕️thickens-stud-shank-to-22-mm` (EN 1994 composite).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1994Snapshot {
    serde_json::from_str(BEFORE).expect("change-d-mm/thickens-stud-shank-to-22-mm: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-d-mm/thickens-stud-shank-to-22-mm: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-d-mm/thickens-stud-shank-to-22-mm: mutation decodes")
}

/// ▶️ `change-d-mm` carries `d_mm` from 19.0 to 22.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-d-mm/thickens-stud-shank-to-22-mm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.d_mm, 22.0, "change-d-mm/thickens-stud-shank-to-22-mm: `d_mm` must read 22.0 after the mutation");
    assert_eq!(produced.h_sc_mm, base.h_sc_mm, "change-d-mm/thickens-stud-shank-to-22-mm: `h_sc_mm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-d-mm/thickens-stud-shank-to-22-mm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `d_mm` (19.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-d-mm/thickens-stud-shank-to-22-mm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-d-mm/thickens-stud-shank-to-22-mm: inverse step applies");
    }
    assert_eq!(snapshot.d_mm, base.d_mm, "change-d-mm/thickens-stud-shank-to-22-mm: inverse must put `d_mm` back to 19.0");
    assert_eq!(snapshot, base, "change-d-mm/thickens-stud-shank-to-22-mm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-d-mm/thickens-stud-shank-to-22-mm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-d-mm/thickens-stud-shank-to-22-mm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-d-mm/thickens-stud-shank-to-22-mm: snapshot reparses");
        assert_eq!(reencoded, original, "change-d-mm/thickens-stud-shank-to-22-mm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-d-mm/thickens-stud-shank-to-22-mm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-d-mm/thickens-stud-shank-to-22-mm: mutation reparses");
    assert_eq!(reencoded, original, "change-d-mm/thickens-stud-shank-to-22-mm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 19.0→22.0 edit of `d_mm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-d-mm/thickens-stud-shank-to-22-mm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-d-mm/thickens-stud-shank-to-22-mm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-d-mm/thickens-stud-shank-to-22-mm: changing `d_mm` away from 19.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-d-mm/thickens-stud-shank-to-22-mm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `dMm` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().d_mm, Some(22.0), "change-d-mm/thickens-stud-shank-to-22-mm: the diff must carry `d_mm` = 22.0");
    assert!(outcome.diff().h_sc_mm.is_none(), "change-d-mm/thickens-stud-shank-to-22-mm: the diff must leave `h_sc_mm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-d-mm/thickens-stud-shank-to-22-mm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-d-mm/thickens-stud-shank-to-22-mm: committed diff decodes");
    assert_eq!(produced, committed, "change-d-mm/thickens-stud-shank-to-22-mm: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-d-mm/thickens-stud-shank-to-22-mm: committed diff decodes");
    assert_eq!(decoded.d_mm, Some(22.0), "change-d-mm/thickens-stud-shank-to-22-mm: the committed diff must name `d_mm` = 22.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-d-mm/thickens-stud-shank-to-22-mm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-d-mm/thickens-stud-shank-to-22-mm: committed diff reparses");
    assert_eq!(reencoded, original, "change-d-mm/thickens-stud-shank-to-22-mm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 22.0 `d_mm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-d-mm/thickens-stud-shank-to-22-mm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-d-mm/thickens-stud-shank-to-22-mm: committed diff applies to the before-snapshot");
    assert_eq!(produced.d_mm, 22.0, "change-d-mm/thickens-stud-shank-to-22-mm: the committed diff must set `d_mm` to 22.0");
    assert_eq!(produced, expected_after(), "change-d-mm/thickens-stud-shank-to-22-mm: committed diff did not carry before to after");
}
