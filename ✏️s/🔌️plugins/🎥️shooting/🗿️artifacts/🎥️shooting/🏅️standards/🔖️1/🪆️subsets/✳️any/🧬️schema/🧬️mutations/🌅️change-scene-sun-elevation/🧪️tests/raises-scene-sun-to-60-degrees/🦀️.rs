//! 🧪️ `change-scene-sun-elevation` fixture — `raises-scene-sun-to-60-degrees`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-scene-sun-elevation diff applies")
}

/// ▶️ `change-scene-sun-elevation` raises the sun above the horizon, in degrees, within a CLOSED
/// ±90 band — the only scene angle that is range-checked.
#[semio_framework_async_macros::async_test]
async fn raises_the_sun_within_the_closed_band() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.scene.sun.elevation, 60.0, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the new elevation is stored verbatim");
    assert_eq!(snapshot.scene.sun.azimuth, before().scene.sun.azimuth, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the compass bearing is a separate field");
    assert_eq!(snapshot.scene.sun.color, before().scene.sun.color, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: raising the sun does not re-tint it");
    assert_eq!(snapshot.scene.material, before().scene.material, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the material block rides along in the cloned scene unchanged");
}

/// ↩️ The inverse re-reads the BASE elevation.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_elevation() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the ±90 band: 90 itself is INSIDE the closed
/// range and applies, while 120 is `mutation.invariant` at Fatal.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_leaving_the_band_is_fatal() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: a real raise must raise no diagnostic");

    let at_the_zenith: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeSceneSunElevation","new_elevation":90.0}"#).expect("probe mutation decodes");
    assert!(at_the_zenith.diff(&before()).messages().is_empty(), "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the band is CLOSED, so exactly 90 is legal");

    let past_the_zenith: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeSceneSunElevation","new_elevation":120.0}"#).expect("probe mutation decodes");
    let rejected = past_the_zenith.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Fatal), "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: leaving the ±90 band must be Fatal");
    assert_eq!(rejected.messages()[0].code.0, "mutation.invariant", "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the range guard's frozen code");
    assert!(rejected.messages()[0].target.is_empty(), "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: a document-root scalar has no addressable target");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins the new elevation inside the cloned scene block while the azimuth it is so easily
/// confused with sits beside it at its base value.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["scene"]["sun"]["elevation"], 60.0, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the edited field inside the cloned scene");
    assert_eq!(committed["scene"]["sun"]["azimuth"], 45.0, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the compass bearing rides along at its BASE value");
    assert_eq!(committed["scene"]["material"]["roughness"], 1.0, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the material block is cloned verbatim, not re-derived");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed whole-scene block round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the cloned scene block is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: committed diff did not carry before to after");
}
