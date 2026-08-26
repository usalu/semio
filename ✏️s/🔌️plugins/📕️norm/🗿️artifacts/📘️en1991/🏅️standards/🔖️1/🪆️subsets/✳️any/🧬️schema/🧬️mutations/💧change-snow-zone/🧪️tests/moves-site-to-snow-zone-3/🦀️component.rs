//! 🧪️ `change-snow-zone` fixture — `moves-site-to-snow-zone-3` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-snow-zone/moves-site-to-snow-zone-3: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-snow-zone/moves-site-to-snow-zone-3: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-snow-zone/moves-site-to-snow-zone-3: mutation decodes")
}

/// ▶️ `change-snow-zone` carries `snow_zone` from 2 to 3 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-snow-zone/moves-site-to-snow-zone-3: mutation applies to its committed before-snapshot");
    assert_eq!(produced.snow_zone, 3, "change-snow-zone/moves-site-to-snow-zone-3: `snow_zone` must read 3 after the mutation");
    assert_eq!(produced.snow_altitude_m, base.snow_altitude_m, "change-snow-zone/moves-site-to-snow-zone-3: `snow_altitude_m` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-snow-zone/moves-site-to-snow-zone-3: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `snow_zone` (2) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-snow-zone/moves-site-to-snow-zone-3: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-snow-zone/moves-site-to-snow-zone-3: inverse step applies");
    }
    assert_eq!(snapshot.snow_zone, base.snow_zone, "change-snow-zone/moves-site-to-snow-zone-3: inverse must put `snow_zone` back to 2");
    assert_eq!(snapshot, base, "change-snow-zone/moves-site-to-snow-zone-3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-snow-zone/moves-site-to-snow-zone-3: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-snow-zone/moves-site-to-snow-zone-3: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-snow-zone/moves-site-to-snow-zone-3: snapshot reparses");
        assert_eq!(reencoded, original, "change-snow-zone/moves-site-to-snow-zone-3: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-snow-zone/moves-site-to-snow-zone-3: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-snow-zone/moves-site-to-snow-zone-3: mutation reparses");
    assert_eq!(reencoded, original, "change-snow-zone/moves-site-to-snow-zone-3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 2→3 edit of `snow_zone` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-snow-zone/moves-site-to-snow-zone-3: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-snow-zone/moves-site-to-snow-zone-3: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-snow-zone/moves-site-to-snow-zone-3: changing `snow_zone` away from 2 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-snow-zone/moves-site-to-snow-zone-3: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `snowZone` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().snow_zone, Some(3), "change-snow-zone/moves-site-to-snow-zone-3: the diff must carry `snow_zone` = 3");
    assert!(outcome.diff().snow_altitude_m.is_none(), "change-snow-zone/moves-site-to-snow-zone-3: the diff must leave `snow_altitude_m` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-snow-zone/moves-site-to-snow-zone-3: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-snow-zone/moves-site-to-snow-zone-3: committed diff decodes");
    assert_eq!(produced, committed, "change-snow-zone/moves-site-to-snow-zone-3: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-snow-zone/moves-site-to-snow-zone-3: committed diff decodes");
    assert_eq!(decoded.snow_zone, Some(3), "change-snow-zone/moves-site-to-snow-zone-3: the committed diff must name `snow_zone` = 3");
    let reencoded = serde_json::to_value(&decoded).expect("change-snow-zone/moves-site-to-snow-zone-3: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-snow-zone/moves-site-to-snow-zone-3: committed diff reparses");
    assert_eq!(reencoded, original, "change-snow-zone/moves-site-to-snow-zone-3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 3 `snow_zone` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-snow-zone/moves-site-to-snow-zone-3: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-snow-zone/moves-site-to-snow-zone-3: committed diff applies to the before-snapshot");
    assert_eq!(produced.snow_zone, 3, "change-snow-zone/moves-site-to-snow-zone-3: the committed diff must set `snow_zone` to 3");
    assert_eq!(produced, expected_after(), "change-snow-zone/moves-site-to-snow-zone-3: committed diff did not carry before to after");
}
