//! 🧪️ `change-scene-material-roughness` fixture — `polishes-scene-material-to-quarter`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-scene-material-roughness diff applies")
}

/// ▶️ `change-scene-material-roughness` writes the one PBR knob this artifact exposes as a
/// mutation. `metalness`, `color`, `emissive` and `emissiveIntensity` sit in the same struct and
/// are all carried through the cloned scene untouched.
#[semio_framework_async_macros::async_test]
async fn polishes_only_the_roughness_knob() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-scene-material-roughness/polishes-scene-material-to-quarter: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.scene.material.roughness, 0.25, "change-scene-material-roughness/polishes-scene-material-to-quarter: the new roughness is stored verbatim");
    assert_eq!(snapshot.scene.material.metalness, before().scene.material.metalness, "change-scene-material-roughness/polishes-scene-material-to-quarter: metalness has no mutation of its own and must not drift");
    assert_eq!(snapshot.scene.material.color, before().scene.material.color, "change-scene-material-roughness/polishes-scene-material-to-quarter: the albedo is untouched");
    assert_eq!(snapshot.scene.sun, before().scene.sun, "change-scene-material-roughness/polishes-scene-material-to-quarter: the sun block rides along in the cloned scene unchanged");
}

/// ↩️ The inverse re-reads the BASE roughness.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_roughness() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-scene-material-roughness/polishes-scene-material-to-quarter: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-scene-material-roughness/polishes-scene-material-to-quarter: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-scene-material-roughness/polishes-scene-material-to-quarter: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the unit-interval invariant: roughness is
/// clamped to a CLOSED `0..=1`, so both ends are legal and anything outside is Fatal.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_leaving_the_unit_interval_is_fatal() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-scene-material-roughness/polishes-scene-material-to-quarter: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-scene-material-roughness/polishes-scene-material-to-quarter: a real polish must raise no diagnostic");

    let mirror: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeSceneMaterialRoughness","new_roughness":0.0}"#).expect("probe mutation decodes");
    assert!(mirror.diff(&before()).messages().is_empty(), "change-scene-material-roughness/polishes-scene-material-to-quarter: the interval is CLOSED, so a perfect mirror at 0 is legal");

    let overshoot: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeSceneMaterialRoughness","new_roughness":1.5}"#).expect("probe mutation decodes");
    let rejected = overshoot.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Fatal), "change-scene-material-roughness/polishes-scene-material-to-quarter: leaving 0..=1 must be Fatal");
    assert_eq!(rejected.messages()[0].code.0, "mutation.invariant", "change-scene-material-roughness/polishes-scene-material-to-quarter: the unit-interval guard's frozen code");
    assert!(rejected.messages()[0].target.is_empty(), "change-scene-material-roughness/polishes-scene-material-to-quarter: a scene-level scalar has no addressable target");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins the one PBR knob that has a mutation, and pins that its neighbours in the same struct —
/// `metalness`, `color`, `emissive` — are cloned rather than defaulted.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-scene-material-roughness/polishes-scene-material-to-quarter: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["scene"]["material"]["roughness"], 0.25, "change-scene-material-roughness/polishes-scene-material-to-quarter: the edited field inside the cloned scene");
    assert_eq!(committed["scene"]["material"]["metalness"], 0.0, "change-scene-material-roughness/polishes-scene-material-to-quarter: metalness has no mutation and must be cloned, not defaulted");
    assert_eq!(committed["scene"]["material"]["emissiveIntensity"], 0.0, "change-scene-material-roughness/polishes-scene-material-to-quarter: the whole material struct rides along, camelCased by the diff's own serde attrs");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed whole-scene block round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-scene-material-roughness/polishes-scene-material-to-quarter: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the cloned scene block is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-scene-material-roughness/polishes-scene-material-to-quarter: committed diff did not carry before to after");
}
