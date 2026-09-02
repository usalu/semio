//! 🧪️ `change-scene-ambient-intensity` fixture — `dims-scene-ambient-to-quarter`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-scene-ambient-intensity diff applies")
}

/// ▶️ `change-scene-ambient-intensity` sets the FILL light's strength. It writes
/// `scene.ambient.intensity` — a different leaf of the cloned scene from the sun's own intensity,
/// which the payload's identically-named `new_intensity` field must not be confused with.
#[semio_framework_async_macros::async_test]
async fn sets_the_fill_light_strength() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.scene.ambient.intensity, 0.25, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the payload value lands on the AMBIENT block");
    assert_eq!(snapshot.scene.sun.intensity, before().scene.sun.intensity, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the sun's own intensity must not move");
    assert_eq!(snapshot.scene.ambient.color, before().scene.ambient.color, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the ambient tint is a separate field");
    assert_eq!(snapshot.scene.shadow, before().scene.shadow, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the shadow block rides along in the cloned scene unchanged");
}

/// ↩️ The inverse re-reads the BASE ambient intensity.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_intensity() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the non-negativity invariant, compared against
/// the AMBIENT base value: 2.4 is the sun's, so setting the ambient to 2.4 is a real change here.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_a_negative_intensity_is_fatal() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: a real dim must raise no diagnostic");

    let negative: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeSceneAmbientIntensity","new_intensity":-0.5}"#).expect("probe mutation decodes");
    let rejected = negative.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Fatal), "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: a negative ambient intensity must be Fatal");
    assert_eq!(rejected.messages()[0].code.0, "mutation.invariant", "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the non-negativity guard's frozen code");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the equality guard compares against `scene.ambient.intensity`, not the sun's");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it is the mirror of the sun-intensity fixture: the AMBIENT intensity is the edited field and the
/// sun's own intensity rides along untouched.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["scene"]["ambient"]["intensity"], 0.25, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the AMBIENT intensity is the edited field");
    assert_eq!(committed["scene"]["sun"]["intensity"], 2.4, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the SUN's intensity rides along at its base value");
    assert_eq!(committed["scene"]["ambient"]["color"], "#ffffff", "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the ambient tint is cloned, not reset");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed whole-scene block round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the cloned scene block is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: committed diff did not carry before to after");
}
