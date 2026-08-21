//! 🧪️ `change-hoisting-speed-ms` fixture — `speeds-hoisting-to-1-25-m-s` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: mutation decodes")
}

/// ▶️ `change-hoisting-speed-ms` carries `hoisting_speed_m_s` from 0.5 to 1.25 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: mutation applies to its committed before-snapshot");
    assert_eq!(produced.hoisting_speed_m_s, 1.25, "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: `hoisting_speed_m_s` must read 1.25 after the mutation");
    assert_eq!(produced.silo_bulk_density_kn_m3, base.silo_bulk_density_kn_m3, "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: `silo_bulk_density_kn_m3` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `hoisting_speed_m_s` (0.5) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: inverse step applies");
    }
    assert_eq!(snapshot.hoisting_speed_m_s, base.hoisting_speed_m_s, "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: inverse must put `hoisting_speed_m_s` back to 0.5");
    assert_eq!(snapshot, base, "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: snapshot reparses");
        assert_eq!(reencoded, original, "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: mutation reparses");
    assert_eq!(reencoded, original, "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 0.5→1.25 edit of `hoisting_speed_m_s` raises no diagnostic.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: changing `hoisting_speed_m_s` away from 0.5 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `hoistingSpeedMS` set, every other field left null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().hoisting_speed_m_s, Some(1.25), "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: the diff must carry `hoisting_speed_m_s` = 1.25");
    assert!(outcome.diff().silo_bulk_density_kn_m3.is_none(), "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: the diff must leave `silo_bulk_density_kn_m3` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: committed diff decodes");
    assert_eq!(produced, committed, "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: committed diff decodes");
    assert_eq!(decoded.hoisting_speed_m_s, Some(1.25), "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: the committed diff must name `hoisting_speed_m_s` = 1.25");
    let reencoded = serde_json::to_value(&decoded).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: committed diff reparses");
    assert_eq!(reencoded, original, "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 1.25 `hoisting_speed_m_s` edit is complete on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: committed diff applies to the before-snapshot");
    assert_eq!(produced.hoisting_speed_m_s, 1.25, "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: the committed diff must set `hoisting_speed_m_s` to 1.25");
    assert_eq!(produced, expected_after(), "change-hoisting-speed-ms/speeds-hoisting-to-1-25-m-s: committed diff did not carry before to after");
}
