//! 🧪️ `change-scene-sun-enabled` fixture — `switches-scene-sun-off`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-scene-sun-enabled diff applies")
}

/// ▶️ Every scene leaf emits a WHOLE cloned `ShootingSceneLighting` as its diff, then edits one
/// field of the clone — so switching the sun off must preserve its azimuth/elevation/intensity for
/// when it is switched back on.
#[semio_framework_async_macros::async_test]
async fn disables_the_sun_but_keeps_its_settings() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-scene-sun-enabled/switches-scene-sun-off: applied state differs from committed after-snapshot");
    assert!(!snapshot.scene.sun.enabled, "change-scene-sun-enabled/switches-scene-sun-off: the sun must be off");
    assert_eq!(snapshot.scene.sun.azimuth, before().scene.sun.azimuth, "change-scene-sun-enabled/switches-scene-sun-off: a disabled sun keeps its azimuth");
    assert_eq!(snapshot.scene.sun.intensity, before().scene.sun.intensity, "change-scene-sun-enabled/switches-scene-sun-off: a disabled sun keeps its intensity");
    assert_eq!(snapshot.scene.shadow, before().scene.shadow, "change-scene-sun-enabled/switches-scene-sun-off: shadows have their own toggle and are not cascaded off");
}

/// ↩️ The inverse re-reads the BASE flag, so it switches the sun back on.
#[semio_framework_async_macros::async_test]
async fn inverse_switches_the_sun_back_on() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-scene-sun-enabled/switches-scene-sun-off: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-scene-sun-enabled/switches-scene-sun-off: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-scene-sun-enabled/switches-scene-sun-off: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — this leaf is a pure boolean toggle: its ONLY guard
/// is the equality `mutation.no-op`, there is no invariant to breach.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_switching_off_twice_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-scene-sun-enabled/switches-scene-sun-off: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-scene-sun-enabled/switches-scene-sun-off: a real toggle must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "change-scene-sun-enabled/switches-scene-sun-off: switching an already-off sun off is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "change-scene-sun-enabled/switches-scene-sun-off: the equality guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "change-scene-sun-enabled/switches-scene-sun-off: a no-op toggle applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it exposes this family's deliberate COARSENESS: the scene leaves ship the whole cloned
/// `ShootingSceneLighting`, so the delta names ambient/shadow/material too — the guarantee is that
/// their values are carried unchanged, not that they are absent.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-scene-sun-enabled/switches-scene-sun-off: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["scene"]["sun"]["enabled"], false, "change-scene-sun-enabled/switches-scene-sun-off: the edited field inside the cloned scene");
    assert_eq!(committed["scene"]["sun"]["intensity"], 2.4, "change-scene-sun-enabled/switches-scene-sun-off: the sun's other settings ride along at their BASE values");
    assert!(committed["assets"].is_null() && committed["shots"].is_null(), "change-scene-sun-enabled/switches-scene-sun-off: coarse within `scene`, but it never leaves it");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed whole-scene block round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-scene-sun-enabled/switches-scene-sun-off: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the cloned scene block is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-scene-sun-enabled/switches-scene-sun-off: committed diff did not carry before to after");
}
