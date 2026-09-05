//! 🧪️ `change-fire-curve` fixture — `🔥️switches-fire-curve-to-hydrocarbon` (EN 1991 actions).
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
    serde_json::from_str(BEFORE).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: mutation decodes")
}

/// ▶️ `change-fire-curve` carries `fire_curve` from FireCurve::Standard to FireCurve::Hydrocarbon and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: mutation applies to its committed before-snapshot");
    assert_eq!(produced.fire_curve, crate::artifacts::en1991::part_1_2::FireCurve::Hydrocarbon, "change-fire-curve/switches-fire-curve-to-hydrocarbon: `fire_curve` must read FireCurve::Hydrocarbon after the mutation");
    assert_eq!(produced.fire_resistance_min, base.fire_resistance_min, "change-fire-curve/switches-fire-curve-to-hydrocarbon: `fire_resistance_min` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-fire-curve/switches-fire-curve-to-hydrocarbon: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `fire_curve` (FireCurve::Standard) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: inverse step applies");
    }
    assert_eq!(snapshot.fire_curve, base.fire_curve, "change-fire-curve/switches-fire-curve-to-hydrocarbon: inverse must put `fire_curve` back to FireCurve::Standard");
    assert_eq!(snapshot, base, "change-fire-curve/switches-fire-curve-to-hydrocarbon: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: snapshot reparses");
        assert_eq!(reencoded, original, "change-fire-curve/switches-fire-curve-to-hydrocarbon: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: mutation reparses");
    assert_eq!(reencoded, original, "change-fire-curve/switches-fire-curve-to-hydrocarbon: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean FireCurve::Standard→FireCurve::Hydrocarbon edit of `fire_curve` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fire-curve/switches-fire-curve-to-hydrocarbon: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-fire-curve/switches-fire-curve-to-hydrocarbon: changing `fire_curve` away from FireCurve::Standard must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-fire-curve/switches-fire-curve-to-hydrocarbon: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `fireCurve` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().fire_curve, Some(crate::artifacts::en1991::part_1_2::FireCurve::Hydrocarbon), "change-fire-curve/switches-fire-curve-to-hydrocarbon: the diff must carry `fire_curve` = FireCurve::Hydrocarbon");
    assert!(outcome.diff().fire_resistance_min.is_none(), "change-fire-curve/switches-fire-curve-to-hydrocarbon: the diff must leave `fire_resistance_min` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: committed diff decodes");
    assert_eq!(produced, committed, "change-fire-curve/switches-fire-curve-to-hydrocarbon: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: committed diff decodes");
    assert_eq!(decoded.fire_curve, Some(crate::artifacts::en1991::part_1_2::FireCurve::Hydrocarbon), "change-fire-curve/switches-fire-curve-to-hydrocarbon: the committed diff must name `fire_curve` = FireCurve::Hydrocarbon");
    let reencoded = serde_json::to_value(&decoded).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: committed diff reparses");
    assert_eq!(reencoded, original, "change-fire-curve/switches-fire-curve-to-hydrocarbon: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the FireCurve::Hydrocarbon `fire_curve` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-fire-curve/switches-fire-curve-to-hydrocarbon: committed diff applies to the before-snapshot");
    assert_eq!(produced.fire_curve, crate::artifacts::en1991::part_1_2::FireCurve::Hydrocarbon, "change-fire-curve/switches-fire-curve-to-hydrocarbon: the committed diff must set `fire_curve` to FireCurve::Hydrocarbon");
    assert_eq!(produced, expected_after(), "change-fire-curve/switches-fire-curve-to-hydrocarbon: committed diff did not carry before to after");
}
