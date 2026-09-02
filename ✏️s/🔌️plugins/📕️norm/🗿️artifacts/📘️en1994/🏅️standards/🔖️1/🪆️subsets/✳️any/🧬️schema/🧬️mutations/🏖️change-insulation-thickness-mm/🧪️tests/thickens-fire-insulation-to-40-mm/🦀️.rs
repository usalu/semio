//! 🧪️ `change-insulation-thickness-mm` fixture — `thickens-fire-insulation-to-40-mm` (EN 1994 composite).
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
    serde_json::from_str(BEFORE).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: mutation decodes")
}

/// ▶️ `change-insulation-thickness-mm` carries `insulation_thickness_mm` from 25.0 to 40.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.insulation_thickness_mm, 40.0, "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: `insulation_thickness_mm` must read 40.0 after the mutation");
    assert_eq!(produced.fire_rating, base.fire_rating, "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: `fire_rating` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `insulation_thickness_mm` (25.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: inverse step applies");
    }
    assert_eq!(snapshot.insulation_thickness_mm, base.insulation_thickness_mm, "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: inverse must put `insulation_thickness_mm` back to 25.0");
    assert_eq!(snapshot, base, "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: snapshot reparses");
        assert_eq!(reencoded, original, "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: mutation reparses");
    assert_eq!(reencoded, original, "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 25.0→40.0 edit of `insulation_thickness_mm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: changing `insulation_thickness_mm` away from 25.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `insulationThicknessMm` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().insulation_thickness_mm, Some(40.0), "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: the diff must carry `insulation_thickness_mm` = 40.0");
    assert!(outcome.diff().fire_rating.is_none(), "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: the diff must leave `fire_rating` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: committed diff decodes");
    assert_eq!(produced, committed, "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: committed diff decodes");
    assert_eq!(decoded.insulation_thickness_mm, Some(40.0), "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: the committed diff must name `insulation_thickness_mm` = 40.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: committed diff reparses");
    assert_eq!(reencoded, original, "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 40.0 `insulation_thickness_mm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: committed diff applies to the before-snapshot");
    assert_eq!(produced.insulation_thickness_mm, 40.0, "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: the committed diff must set `insulation_thickness_mm` to 40.0");
    assert_eq!(produced, expected_after(), "change-insulation-thickness-mm/thickens-fire-insulation-to-40-mm: committed diff did not carry before to after");
}
