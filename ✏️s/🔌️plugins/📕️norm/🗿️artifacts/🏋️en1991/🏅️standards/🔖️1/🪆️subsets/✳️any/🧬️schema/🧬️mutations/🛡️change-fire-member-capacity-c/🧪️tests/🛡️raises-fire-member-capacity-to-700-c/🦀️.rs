//! 🧪️ `change-fire-member-capacity-c` fixture — `🛡️raises-fire-member-capacity-to-700-c` (EN 1991 actions).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1991Snapshot {
    serde_json::from_str(BEFORE).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: mutation decodes")
}

/// ▶️ `change-fire-member-capacity-c` carries `fire_member_capacity_c` from 550.0 to 700.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: mutation applies to its committed before-snapshot");
    assert_eq!(produced.fire_member_capacity_c, 700.0, "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: `fire_member_capacity_c` must read 700.0 after the mutation");
    assert_eq!(produced.snow_zone, base.snow_zone, "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: `snow_zone` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `fire_member_capacity_c` (550.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: inverse step applies");
    }
    assert_eq!(snapshot.fire_member_capacity_c, base.fire_member_capacity_c, "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: inverse must put `fire_member_capacity_c` back to 550.0");
    assert_eq!(snapshot, base, "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: snapshot reparses");
        assert_eq!(reencoded, original, "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: mutation reparses");
    assert_eq!(reencoded, original, "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 550.0→700.0 edit of `fire_member_capacity_c` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: changing `fire_member_capacity_c` away from 550.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `fireMemberCapacityC` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().fire_member_capacity_c, Some(700.0), "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: the diff must carry `fire_member_capacity_c` = 700.0");
    assert!(outcome.diff().snow_zone.is_none(), "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: the diff must leave `snow_zone` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: committed diff decodes");
    assert_eq!(produced, committed, "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: committed diff decodes");
    assert_eq!(decoded.fire_member_capacity_c, Some(700.0), "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: the committed diff must name `fire_member_capacity_c` = 700.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: committed diff reparses");
    assert_eq!(reencoded, original, "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 700.0 `fire_member_capacity_c` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: committed diff applies to the before-snapshot");
    assert_eq!(produced.fire_member_capacity_c, 700.0, "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: the committed diff must set `fire_member_capacity_c` to 700.0");
    assert_eq!(produced, expected_after(), "change-fire-member-capacity-c/raises-fire-member-capacity-to-700-c: committed diff did not carry before to after");
}
