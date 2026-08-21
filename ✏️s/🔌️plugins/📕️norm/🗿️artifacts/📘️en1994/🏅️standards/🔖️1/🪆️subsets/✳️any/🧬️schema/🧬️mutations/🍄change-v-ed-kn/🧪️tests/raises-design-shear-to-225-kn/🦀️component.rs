//! 🧪️ `change-v-ed-kn` fixture — `raises-design-shear-to-225-kn` (EN 1994 composite).
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
    serde_json::from_str(BEFORE).expect("change-v-ed-kn/raises-design-shear-to-225-kn: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-v-ed-kn/raises-design-shear-to-225-kn: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-v-ed-kn/raises-design-shear-to-225-kn: mutation decodes")
}

/// ▶️ `change-v-ed-kn` carries `v_ed_kn` from 150.0 to 225.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-v-ed-kn/raises-design-shear-to-225-kn: mutation applies to its committed before-snapshot");
    assert_eq!(produced.v_ed_kn, 225.0, "change-v-ed-kn/raises-design-shear-to-225-kn: `v_ed_kn` must read 225.0 after the mutation");
    assert_eq!(produced.m_pla, base.m_pla, "change-v-ed-kn/raises-design-shear-to-225-kn: `m_pla` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-v-ed-kn/raises-design-shear-to-225-kn: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `v_ed_kn` (150.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-v-ed-kn/raises-design-shear-to-225-kn: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-v-ed-kn/raises-design-shear-to-225-kn: inverse step applies");
    }
    assert_eq!(snapshot.v_ed_kn, base.v_ed_kn, "change-v-ed-kn/raises-design-shear-to-225-kn: inverse must put `v_ed_kn` back to 150.0");
    assert_eq!(snapshot, base, "change-v-ed-kn/raises-design-shear-to-225-kn: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-v-ed-kn/raises-design-shear-to-225-kn: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-v-ed-kn/raises-design-shear-to-225-kn: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-v-ed-kn/raises-design-shear-to-225-kn: snapshot reparses");
        assert_eq!(reencoded, original, "change-v-ed-kn/raises-design-shear-to-225-kn: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-v-ed-kn/raises-design-shear-to-225-kn: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-v-ed-kn/raises-design-shear-to-225-kn: mutation reparses");
    assert_eq!(reencoded, original, "change-v-ed-kn/raises-design-shear-to-225-kn: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 150.0→225.0 edit of `v_ed_kn` raises no diagnostic.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-v-ed-kn/raises-design-shear-to-225-kn: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-v-ed-kn/raises-design-shear-to-225-kn: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-v-ed-kn/raises-design-shear-to-225-kn: changing `v_ed_kn` away from 150.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-v-ed-kn/raises-design-shear-to-225-kn: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `vEdKn` set, every other field left null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().v_ed_kn, Some(225.0), "change-v-ed-kn/raises-design-shear-to-225-kn: the diff must carry `v_ed_kn` = 225.0");
    assert!(outcome.diff().m_pla.is_none(), "change-v-ed-kn/raises-design-shear-to-225-kn: the diff must leave `m_pla` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-v-ed-kn/raises-design-shear-to-225-kn: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-v-ed-kn/raises-design-shear-to-225-kn: committed diff decodes");
    assert_eq!(produced, committed, "change-v-ed-kn/raises-design-shear-to-225-kn: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-v-ed-kn/raises-design-shear-to-225-kn: committed diff decodes");
    assert_eq!(decoded.v_ed_kn, Some(225.0), "change-v-ed-kn/raises-design-shear-to-225-kn: the committed diff must name `v_ed_kn` = 225.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-v-ed-kn/raises-design-shear-to-225-kn: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-v-ed-kn/raises-design-shear-to-225-kn: committed diff reparses");
    assert_eq!(reencoded, original, "change-v-ed-kn/raises-design-shear-to-225-kn: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 225.0 `v_ed_kn` edit is complete on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-v-ed-kn/raises-design-shear-to-225-kn: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-v-ed-kn/raises-design-shear-to-225-kn: committed diff applies to the before-snapshot");
    assert_eq!(produced.v_ed_kn, 225.0, "change-v-ed-kn/raises-design-shear-to-225-kn: the committed diff must set `v_ed_kn` to 225.0");
    assert_eq!(produced, expected_after(), "change-v-ed-kn/raises-design-shear-to-225-kn: committed diff did not carry before to after");
}
