//! 🧪️ `change-v-weld-ed-kn` fixture — `✂️raises-weld-shear-to-48-kn` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: mutation decodes")
}

/// ▶️ `change-v-weld-ed-kn` carries `v_weld_ed_kn` from 32.0 to 48.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: mutation applies to its committed before-snapshot");
    assert_eq!(produced.v_weld_ed_kn, 48.0, "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: `v_weld_ed_kn` must read 48.0 after the mutation");
    assert_eq!(produced.weld_throat_mm, base.weld_throat_mm, "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: `weld_throat_mm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `v_weld_ed_kn` (32.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: inverse step applies");
    }
    assert_eq!(snapshot.v_weld_ed_kn, base.v_weld_ed_kn, "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: inverse must put `v_weld_ed_kn` back to 32.0");
    assert_eq!(snapshot, base, "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: snapshot reparses");
        assert_eq!(reencoded, original, "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: mutation reparses");
    assert_eq!(reencoded, original, "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 32.0→48.0 edit of `v_weld_ed_kn` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: changing `v_weld_ed_kn` away from 32.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `vWeldEdKn` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().v_weld_ed_kn, Some(48.0), "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: the diff must carry `v_weld_ed_kn` = 48.0");
    assert!(outcome.diff().weld_throat_mm.is_none(), "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: the diff must leave `weld_throat_mm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: committed diff decodes");
    assert_eq!(produced, committed, "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: committed diff decodes");
    assert_eq!(decoded.v_weld_ed_kn, Some(48.0), "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: the committed diff must name `v_weld_ed_kn` = 48.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: committed diff reparses");
    assert_eq!(reencoded, original, "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 48.0 `v_weld_ed_kn` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-v-weld-ed-kn/raises-weld-shear-to-48-kn: committed diff applies to the before-snapshot");
    assert_eq!(produced.v_weld_ed_kn, 48.0, "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: the committed diff must set `v_weld_ed_kn` to 48.0");
    assert_eq!(produced, expected_after(), "change-v-weld-ed-kn/raises-weld-shear-to-48-kn: committed diff did not carry before to after");
}
