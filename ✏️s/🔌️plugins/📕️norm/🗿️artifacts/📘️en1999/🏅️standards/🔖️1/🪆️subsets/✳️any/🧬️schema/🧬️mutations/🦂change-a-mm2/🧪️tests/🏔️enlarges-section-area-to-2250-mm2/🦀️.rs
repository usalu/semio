//! 🧪️ `change-a-mm2` fixture — `🏔️enlarges-section-area-to-2250-mm2` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: mutation decodes")
}

/// ▶️ `change-a-mm2` carries `a_mm2` from 1500.0 to 2250.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: mutation applies to its committed before-snapshot");
    assert_eq!(produced.a_mm2, 2250.0, "change-a-mm2/enlarges-section-area-to-2250-mm2: `a_mm2` must read 2250.0 after the mutation");
    assert_eq!(produced.w_el_mm3, base.w_el_mm3, "change-a-mm2/enlarges-section-area-to-2250-mm2: `w_el_mm3` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-a-mm2/enlarges-section-area-to-2250-mm2: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `a_mm2` (1500.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: inverse step applies");
    }
    assert_eq!(snapshot.a_mm2, base.a_mm2, "change-a-mm2/enlarges-section-area-to-2250-mm2: inverse must put `a_mm2` back to 1500.0");
    assert_eq!(snapshot, base, "change-a-mm2/enlarges-section-area-to-2250-mm2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: snapshot reparses");
        assert_eq!(reencoded, original, "change-a-mm2/enlarges-section-area-to-2250-mm2: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: mutation reparses");
    assert_eq!(reencoded, original, "change-a-mm2/enlarges-section-area-to-2250-mm2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 1500.0→2250.0 edit of `a_mm2` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-a-mm2/enlarges-section-area-to-2250-mm2: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-a-mm2/enlarges-section-area-to-2250-mm2: changing `a_mm2` away from 1500.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-a-mm2/enlarges-section-area-to-2250-mm2: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `aMm2` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().a_mm2, Some(2250.0), "change-a-mm2/enlarges-section-area-to-2250-mm2: the diff must carry `a_mm2` = 2250.0");
    assert!(outcome.diff().w_el_mm3.is_none(), "change-a-mm2/enlarges-section-area-to-2250-mm2: the diff must leave `w_el_mm3` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: committed diff decodes");
    assert_eq!(produced, committed, "change-a-mm2/enlarges-section-area-to-2250-mm2: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: committed diff decodes");
    assert_eq!(decoded.a_mm2, Some(2250.0), "change-a-mm2/enlarges-section-area-to-2250-mm2: the committed diff must name `a_mm2` = 2250.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: committed diff reparses");
    assert_eq!(reencoded, original, "change-a-mm2/enlarges-section-area-to-2250-mm2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 2250.0 `a_mm2` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-a-mm2/enlarges-section-area-to-2250-mm2: committed diff applies to the before-snapshot");
    assert_eq!(produced.a_mm2, 2250.0, "change-a-mm2/enlarges-section-area-to-2250-mm2: the committed diff must set `a_mm2` to 2250.0");
    assert_eq!(produced, expected_after(), "change-a-mm2/enlarges-section-area-to-2250-mm2: committed diff did not carry before to after");
}
