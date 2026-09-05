//! 🧪️ `change-scene-sun-azimuth` fixture — `🧭️turns-scene-sun-to-315-degrees`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingDiff, ShootingSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> ShootingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> ShootingSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> ShootingMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn apply(base: &ShootingSnapshot, step: &ShootingMutation) -> ShootingSnapshot {
    step.diff(base).into_parts().0.apply(base).expect("change-scene-sun-azimuth diff applies")
}

/// ▶️ `change-scene-sun-azimuth` writes the compass bearing in DEGREES. Unlike elevation it has no
/// range clamp at all — 315 is stored as given, never wrapped or normalized into ±180.
#[semio_framework_async_macros::async_test]
async fn stores_the_bearing_in_degrees_unwrapped() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.scene.sun.azimuth, 315.0, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: the bearing is stored verbatim, not wrapped to -45");
    assert_eq!(snapshot.scene.sun.elevation, before().scene.sun.elevation, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: the sun's height above the horizon is a separate field");
    assert!(snapshot.scene.sun.enabled, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: turning the sun does not toggle it");
    assert_eq!(snapshot.scene.ambient, before().scene.ambient, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: the ambient block rides along in the cloned scene unchanged");
}

/// ↩️ The inverse re-reads the BASE azimuth.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_bearing() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — this leaf's `mutation.invariant` guard only rejects
/// NON-FINITE bearings, which JSON cannot even express, so the only reachable guard from a
/// committed payload is the equality `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_an_unchanged_bearing_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: a real turn must raise no diagnostic");

    let beyond_a_full_turn: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeSceneSunAzimuth","new_azimuth":720.0}"#).expect("probe mutation decodes");
    assert!(beyond_a_full_turn.diff(&before()).messages().is_empty(), "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: azimuth is unbounded — 720 degrees is accepted, unlike elevation's ±90 clamp");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: an unchanged bearing is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: the equality guard's frozen code");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins the unwrapped bearing INSIDE the cloned scene block, and pins that the sun's enabled
/// flag and elevation ride along at their base values rather than being recomputed.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["scene"]["sun"]["azimuth"], 315.0, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: the bearing is in the delta unwrapped, not normalized to -45");
    assert_eq!(committed["scene"]["sun"]["elevation"], 35.0, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: elevation rides along at its BASE value");
    assert!(committed["activeShotId"].is_null() && committed["camera"].is_null(), "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: turning the sun touches no cursor and no config camera");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed whole-scene block round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the cloned scene block is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: committed diff did not carry before to after");
}
