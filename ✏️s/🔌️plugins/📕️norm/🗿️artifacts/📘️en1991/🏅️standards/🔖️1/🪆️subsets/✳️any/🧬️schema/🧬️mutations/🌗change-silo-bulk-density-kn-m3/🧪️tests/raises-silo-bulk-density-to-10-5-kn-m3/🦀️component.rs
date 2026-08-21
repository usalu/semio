//! 🧪️ `change-silo-bulk-density-kn-m3` fixture — `raises-silo-bulk-density-to-10-5-kn-m3` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: mutation decodes")
}

/// ▶️ `change-silo-bulk-density-kn-m3` carries `silo_bulk_density_kn_m3` from 8.5 to 10.5 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: mutation applies to its committed before-snapshot");
    assert_eq!(produced.silo_bulk_density_kn_m3, 10.5, "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: `silo_bulk_density_kn_m3` must read 10.5 after the mutation");
    assert_eq!(produced.silo_height_m, base.silo_height_m, "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: `silo_height_m` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `silo_bulk_density_kn_m3` (8.5) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: inverse step applies");
    }
    assert_eq!(snapshot.silo_bulk_density_kn_m3, base.silo_bulk_density_kn_m3, "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: inverse must put `silo_bulk_density_kn_m3` back to 8.5");
    assert_eq!(snapshot, base, "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: snapshot reparses");
        assert_eq!(reencoded, original, "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: mutation reparses");
    assert_eq!(reencoded, original, "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 8.5→10.5 edit of `silo_bulk_density_kn_m3` raises no diagnostic.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: changing `silo_bulk_density_kn_m3` away from 8.5 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `siloBulkDensityKnM3` set, every other field left null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().silo_bulk_density_kn_m3, Some(10.5), "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: the diff must carry `silo_bulk_density_kn_m3` = 10.5");
    assert!(outcome.diff().silo_height_m.is_none(), "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: the diff must leave `silo_height_m` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: committed diff decodes");
    assert_eq!(produced, committed, "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: committed diff decodes");
    assert_eq!(decoded.silo_bulk_density_kn_m3, Some(10.5), "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: the committed diff must name `silo_bulk_density_kn_m3` = 10.5");
    let reencoded = serde_json::to_value(&decoded).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: committed diff reparses");
    assert_eq!(reencoded, original, "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 10.5 `silo_bulk_density_kn_m3` edit is complete on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: committed diff applies to the before-snapshot");
    assert_eq!(produced.silo_bulk_density_kn_m3, 10.5, "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: the committed diff must set `silo_bulk_density_kn_m3` to 10.5");
    assert_eq!(produced, expected_after(), "change-silo-bulk-density-kn-m3/raises-silo-bulk-density-to-10-5-kn-m3: committed diff did not carry before to after");
}
