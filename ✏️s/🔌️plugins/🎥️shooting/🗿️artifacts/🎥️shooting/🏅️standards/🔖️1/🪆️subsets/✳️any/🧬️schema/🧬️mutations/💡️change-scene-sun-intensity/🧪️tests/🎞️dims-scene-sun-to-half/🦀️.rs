//! 🧪️ `change-scene-sun-intensity` fixture — `🎞️dims-scene-sun-to-half`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-scene-sun-intensity diff applies")
}

/// ▶️ `change-scene-sun-intensity` sets the KEY light's strength absolutely — it is not a factor
/// applied to the old value — and never touches the ambient fill.
#[semio_framework_async_macros::async_test]
async fn sets_the_key_light_strength_absolutely() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-scene-sun-intensity/dims-scene-sun-to-half: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.scene.sun.intensity, 1.2, "change-scene-sun-intensity/dims-scene-sun-to-half: the payload value replaces the old strength outright");
    assert_eq!(snapshot.scene.ambient.intensity, before().scene.ambient.intensity, "change-scene-sun-intensity/dims-scene-sun-to-half: the ambient fill has its own mutation");
    assert!(snapshot.scene.sun.enabled, "change-scene-sun-intensity/dims-scene-sun-to-half: dimming is not disabling");
    assert_eq!(snapshot.scene.background, before().scene.background, "change-scene-sun-intensity/dims-scene-sun-to-half: the scene background rides along in the cloned scene unchanged");
}

/// ↩️ The inverse re-reads the BASE intensity.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_intensity() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-scene-sun-intensity/dims-scene-sun-to-half: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-scene-sun-intensity/dims-scene-sun-to-half: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-scene-sun-intensity/dims-scene-sun-to-half: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the non-negativity invariant: intensity is
/// bounded BELOW at zero (zero itself is legal) and unbounded above.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_a_negative_intensity_is_fatal() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-scene-sun-intensity/dims-scene-sun-to-half: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-scene-sun-intensity/dims-scene-sun-to-half: a real dim must raise no diagnostic");

    let extinguished: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeSceneSunIntensity","new_intensity":0.0}"#).expect("probe mutation decodes");
    assert!(extinguished.diff(&before()).messages().is_empty(), "change-scene-sun-intensity/dims-scene-sun-to-half: zero is a legal intensity, the guard rejects only NEGATIVE values");

    let negative: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeSceneSunIntensity","new_intensity":-1.0}"#).expect("probe mutation decodes");
    let rejected = negative.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Fatal), "change-scene-sun-intensity/dims-scene-sun-to-half: a negative intensity must be Fatal");
    assert_eq!(rejected.messages()[0].code.0, "mutation.invariant", "change-scene-sun-intensity/dims-scene-sun-to-half: the non-negativity guard's frozen code");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins the KEY light's new strength and — crucially — the ambient fill's untouched strength in
/// the same block, the only place those two identically-named payload fields are distinguishable.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-scene-sun-intensity/dims-scene-sun-to-half: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["scene"]["sun"]["intensity"], 1.2, "change-scene-sun-intensity/dims-scene-sun-to-half: the SUN's intensity is the edited field");
    assert_eq!(committed["scene"]["ambient"]["intensity"], 1.15, "change-scene-sun-intensity/dims-scene-sun-to-half: the AMBIENT intensity rides along at its base value");
    assert_eq!(committed["scene"]["sun"]["enabled"], true, "change-scene-sun-intensity/dims-scene-sun-to-half: dimming is not disabling, even at delta level");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed whole-scene block round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-scene-sun-intensity/dims-scene-sun-to-half: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the cloned scene block is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-scene-sun-intensity/dims-scene-sun-to-half: committed diff did not carry before to after");
}
