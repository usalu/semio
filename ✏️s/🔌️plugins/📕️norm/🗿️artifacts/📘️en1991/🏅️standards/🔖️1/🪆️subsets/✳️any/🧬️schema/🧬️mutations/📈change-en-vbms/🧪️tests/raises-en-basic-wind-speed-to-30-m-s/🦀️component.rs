//! 🧪️ `change-en-vbms` fixture — `raises-en-basic-wind-speed-to-30-m-s` (EN 1991 actions).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1991Snapshot {
    serde_json::from_str(BEFORE).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: mutation decodes")
}

/// ▶️ `change-en-vbms` carries `en_v_b_m_s` from 25.0 to 30.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: mutation applies to its committed before-snapshot");
    assert_eq!(produced.en_v_b_m_s, 30.0, "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: `en_v_b_m_s` must read 30.0 after the mutation");
    assert_eq!(produced.delta_t_k, base.delta_t_k, "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: `delta_t_k` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `en_v_b_m_s` (25.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: inverse step applies");
    }
    assert_eq!(snapshot.en_v_b_m_s, base.en_v_b_m_s, "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: inverse must put `en_v_b_m_s` back to 25.0");
    assert_eq!(snapshot, base, "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: snapshot reparses");
        assert_eq!(reencoded, original, "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: mutation reparses");
    assert_eq!(reencoded, original, "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 25.0→30.0 edit of `en_v_b_m_s` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: changing `en_v_b_m_s` away from 25.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `enVBMS` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().en_v_b_m_s, Some(30.0), "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: the diff must carry `en_v_b_m_s` = 30.0");
    assert!(outcome.diff().delta_t_k.is_none(), "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: the diff must leave `delta_t_k` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: committed diff decodes");
    assert_eq!(produced, committed, "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: committed diff decodes");
    assert_eq!(decoded.en_v_b_m_s, Some(30.0), "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: the committed diff must name `en_v_b_m_s` = 30.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: committed diff reparses");
    assert_eq!(reencoded, original, "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 30.0 `en_v_b_m_s` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: committed diff applies to the before-snapshot");
    assert_eq!(produced.en_v_b_m_s, 30.0, "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: the committed diff must set `en_v_b_m_s` to 30.0");
    assert_eq!(produced, expected_after(), "change-en-vbms/raises-en-basic-wind-speed-to-30-m-s: committed diff did not carry before to after");
}
