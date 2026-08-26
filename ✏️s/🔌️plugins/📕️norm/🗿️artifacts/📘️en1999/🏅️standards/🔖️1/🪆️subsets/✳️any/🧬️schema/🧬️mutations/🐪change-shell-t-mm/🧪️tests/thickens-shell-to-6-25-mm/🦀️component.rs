//! 🧪️ `change-shell-t-mm` fixture — `thickens-shell-to-6-25-mm` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: mutation decodes")
}

/// ▶️ `change-shell-t-mm` carries `shell_t_mm` from 5.0 to 6.25 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.shell_t_mm, 6.25, "change-shell-t-mm/thickens-shell-to-6-25-mm: `shell_t_mm` must read 6.25 after the mutation");
    assert_eq!(produced.shell_r_mm, base.shell_r_mm, "change-shell-t-mm/thickens-shell-to-6-25-mm: `shell_r_mm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-shell-t-mm/thickens-shell-to-6-25-mm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `shell_t_mm` (5.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: inverse step applies");
    }
    assert_eq!(snapshot.shell_t_mm, base.shell_t_mm, "change-shell-t-mm/thickens-shell-to-6-25-mm: inverse must put `shell_t_mm` back to 5.0");
    assert_eq!(snapshot, base, "change-shell-t-mm/thickens-shell-to-6-25-mm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: snapshot reparses");
        assert_eq!(reencoded, original, "change-shell-t-mm/thickens-shell-to-6-25-mm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: mutation reparses");
    assert_eq!(reencoded, original, "change-shell-t-mm/thickens-shell-to-6-25-mm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 5.0→6.25 edit of `shell_t_mm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-shell-t-mm/thickens-shell-to-6-25-mm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-shell-t-mm/thickens-shell-to-6-25-mm: changing `shell_t_mm` away from 5.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-shell-t-mm/thickens-shell-to-6-25-mm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `shellTMm` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().shell_t_mm, Some(6.25), "change-shell-t-mm/thickens-shell-to-6-25-mm: the diff must carry `shell_t_mm` = 6.25");
    assert!(outcome.diff().shell_r_mm.is_none(), "change-shell-t-mm/thickens-shell-to-6-25-mm: the diff must leave `shell_r_mm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: committed diff decodes");
    assert_eq!(produced, committed, "change-shell-t-mm/thickens-shell-to-6-25-mm: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: committed diff decodes");
    assert_eq!(decoded.shell_t_mm, Some(6.25), "change-shell-t-mm/thickens-shell-to-6-25-mm: the committed diff must name `shell_t_mm` = 6.25");
    let reencoded = serde_json::to_value(&decoded).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: committed diff reparses");
    assert_eq!(reencoded, original, "change-shell-t-mm/thickens-shell-to-6-25-mm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 6.25 `shell_t_mm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-shell-t-mm/thickens-shell-to-6-25-mm: committed diff applies to the before-snapshot");
    assert_eq!(produced.shell_t_mm, 6.25, "change-shell-t-mm/thickens-shell-to-6-25-mm: the committed diff must set `shell_t_mm` to 6.25");
    assert_eq!(produced, expected_after(), "change-shell-t-mm/thickens-shell-to-6-25-mm: committed diff did not carry before to after");
}
