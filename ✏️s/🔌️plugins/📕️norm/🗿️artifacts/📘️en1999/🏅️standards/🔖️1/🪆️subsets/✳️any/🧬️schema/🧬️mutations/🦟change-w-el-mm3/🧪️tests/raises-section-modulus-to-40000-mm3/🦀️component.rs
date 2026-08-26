//! 🧪️ `change-w-el-mm3` fixture — `raises-section-modulus-to-40000-mm3` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: mutation decodes")
}

/// ▶️ `change-w-el-mm3` carries `w_el_mm3` from 30000.0 to 40000.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: mutation applies to its committed before-snapshot");
    assert_eq!(produced.w_el_mm3, 40000.0, "change-w-el-mm3/raises-section-modulus-to-40000-mm3: `w_el_mm3` must read 40000.0 after the mutation");
    assert_eq!(produced.alloy, base.alloy, "change-w-el-mm3/raises-section-modulus-to-40000-mm3: `alloy` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-w-el-mm3/raises-section-modulus-to-40000-mm3: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `w_el_mm3` (30000.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: inverse step applies");
    }
    assert_eq!(snapshot.w_el_mm3, base.w_el_mm3, "change-w-el-mm3/raises-section-modulus-to-40000-mm3: inverse must put `w_el_mm3` back to 30000.0");
    assert_eq!(snapshot, base, "change-w-el-mm3/raises-section-modulus-to-40000-mm3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: snapshot reparses");
        assert_eq!(reencoded, original, "change-w-el-mm3/raises-section-modulus-to-40000-mm3: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: mutation reparses");
    assert_eq!(reencoded, original, "change-w-el-mm3/raises-section-modulus-to-40000-mm3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 30000.0→40000.0 edit of `w_el_mm3` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-w-el-mm3/raises-section-modulus-to-40000-mm3: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-w-el-mm3/raises-section-modulus-to-40000-mm3: changing `w_el_mm3` away from 30000.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-w-el-mm3/raises-section-modulus-to-40000-mm3: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `wElMm3` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().w_el_mm3, Some(40000.0), "change-w-el-mm3/raises-section-modulus-to-40000-mm3: the diff must carry `w_el_mm3` = 40000.0");
    assert!(outcome.diff().alloy.is_none(), "change-w-el-mm3/raises-section-modulus-to-40000-mm3: the diff must leave `alloy` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: committed diff decodes");
    assert_eq!(produced, committed, "change-w-el-mm3/raises-section-modulus-to-40000-mm3: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: committed diff decodes");
    assert_eq!(decoded.w_el_mm3, Some(40000.0), "change-w-el-mm3/raises-section-modulus-to-40000-mm3: the committed diff must name `w_el_mm3` = 40000.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: committed diff reparses");
    assert_eq!(reencoded, original, "change-w-el-mm3/raises-section-modulus-to-40000-mm3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 40000.0 `w_el_mm3` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-w-el-mm3/raises-section-modulus-to-40000-mm3: committed diff applies to the before-snapshot");
    assert_eq!(produced.w_el_mm3, 40000.0, "change-w-el-mm3/raises-section-modulus-to-40000-mm3: the committed diff must set `w_el_mm3` to 40000.0");
    assert_eq!(produced, expected_after(), "change-w-el-mm3/raises-section-modulus-to-40000-mm3: committed diff did not carry before to after");
}
