//! 🧪️ `change-scene-shadow-enabled` fixture — `switches-scene-shadows-off`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingDiff, ShootingSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

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
    step.diff(base).into_parts().0.apply(base).expect("change-scene-shadow-enabled diff applies")
}

/// ▶️ `change-scene-shadow-enabled` toggles `scene.shadow.enabled` only — the shadow's `opacity`
/// and `softness` are preserved so the old look returns when it is switched back on, and the SUN's
/// own toggle is left alone even though the shadow is cast by it.
#[semio_framework_async_macros::async_test]
async fn disables_shadows_but_keeps_their_settings() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-scene-shadow-enabled/switches-scene-shadows-off: applied state differs from committed after-snapshot");
    assert!(!snapshot.scene.shadow.enabled, "change-scene-shadow-enabled/switches-scene-shadows-off: shadows must be off");
    assert_eq!(snapshot.scene.shadow.opacity, before().scene.shadow.opacity, "change-scene-shadow-enabled/switches-scene-shadows-off: disabled shadows keep their opacity");
    assert_eq!(snapshot.scene.shadow.softness, before().scene.shadow.softness, "change-scene-shadow-enabled/switches-scene-shadows-off: disabled shadows keep their softness");
    assert!(snapshot.scene.sun.enabled, "change-scene-shadow-enabled/switches-scene-shadows-off: the sun that casts them stays on");
}

/// ↩️ The inverse re-reads the BASE flag, so it switches shadows back on.
#[semio_framework_async_macros::async_test]
async fn inverse_switches_shadows_back_on() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-scene-shadow-enabled/switches-scene-shadows-off: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-scene-shadow-enabled/switches-scene-shadows-off: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-scene-shadow-enabled/switches-scene-shadows-off: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — a pure boolean toggle whose only guard is the
/// equality `mutation.no-op`, compared against `scene.shadow.enabled` rather than the sun's flag.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_switching_off_twice_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-scene-shadow-enabled/switches-scene-shadows-off: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-scene-shadow-enabled/switches-scene-shadows-off: a real toggle must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "change-scene-shadow-enabled/switches-scene-shadows-off: switching already-off shadows off is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "change-scene-shadow-enabled/switches-scene-shadows-off: the equality guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "change-scene-shadow-enabled/switches-scene-shadows-off: a no-op toggle applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins the shadow toggle inside the cloned scene block together with the sun's own flag, which
/// stays TRUE in the same delta — the two toggles are not chained.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-scene-shadow-enabled/switches-scene-shadows-off: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["scene"]["shadow"]["enabled"], false, "change-scene-shadow-enabled/switches-scene-shadows-off: the edited field inside the cloned scene");
    assert_eq!(committed["scene"]["sun"]["enabled"], true, "change-scene-shadow-enabled/switches-scene-shadows-off: the sun that casts the shadow stays on in the same delta");
    assert_eq!(committed["scene"]["shadow"]["opacity"], 0.35, "change-scene-shadow-enabled/switches-scene-shadows-off: opacity rides along so the old look returns on re-enable");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed whole-scene block round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-scene-shadow-enabled/switches-scene-shadows-off: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the cloned scene block is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-scene-shadow-enabled/switches-scene-shadows-off: committed diff did not carry before to after");
}
