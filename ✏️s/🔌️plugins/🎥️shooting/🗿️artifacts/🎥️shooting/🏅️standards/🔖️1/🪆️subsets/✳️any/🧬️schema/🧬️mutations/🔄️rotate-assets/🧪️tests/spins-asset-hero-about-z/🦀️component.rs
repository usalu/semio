//! 🧪️ `rotate-assets` fixture — `spins-asset-hero-about-z`.
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
    step.diff(base).into_parts().0.apply(base).expect("rotate-assets diff applies")
}

/// ▶️ `rotate-assets` PRE-multiplies the axis-angle delta onto each asset's current orientation
/// (`delta * current`). "asset-hero" starts at the identity quaternion, so the stored result is the
/// delta itself: a 1.5 rad turn about +z is `[0, 0, sin(0.75), cos(0.75)]`.
#[semio_framework_async_macros::async_test]
async fn composes_the_axis_angle_delta_onto_the_current_orientation() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "rotate-assets/spins-asset-hero-about-z: applied state differs from committed after-snapshot");
    let orientation = snapshot.assets[0].orientation.expect("rotate-assets always writes a concrete quaternion");
    assert_eq!(orientation[0], 0.0, "rotate-assets/spins-asset-hero-about-z: a +z spin leaves the x component at zero");
    assert_eq!(orientation[1], 0.0, "rotate-assets/spins-asset-hero-about-z: a +z spin leaves the y component at zero");
    assert_eq!(orientation[2], (1.5f64 * 0.5).sin(), "rotate-assets/spins-asset-hero-about-z: the z component is sin(angle/2) along the normalized axis");
    assert_eq!(orientation[3], (1.5f64 * 0.5).cos(), "rotate-assets/spins-asset-hero-about-z: the w component is cos(angle/2)");
    assert_eq!(snapshot.assets[0].origin, before().assets[0].origin, "rotate-assets/spins-asset-hero-about-z: a rotation never moves the origin");
    assert_eq!(snapshot.assets[1], before().assets[1], "rotate-assets/spins-asset-hero-about-z: the unaddressed asset keeps its absent orientation");
}

/// ↩️ The inverse re-uses the SAME axis with a negated angle, and the composition unwinds exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_spins_back_about_the_same_axis() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "rotate-assets/spins-asset-hero-about-z: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rotate-assets/spins-asset-hero-about-z: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rotate-assets/spins-asset-hero-about-z: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the degenerate-axis guard: a zero-length axis
/// falls back to the identity quaternion, so the rotation applies but changes nothing.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_a_zero_length_axis_is_the_identity() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rotate-assets/spins-asset-hero-about-z: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "rotate-assets/spins-asset-hero-about-z: a resolvable finite rotation must raise no diagnostic");

    let degenerate: ShootingMutation = serde_json::from_str(r#"{"mutation":"rotateAssets","asset_ids":["asset-hero"],"ax":0.0,"ay":0.0,"az":0.0,"angle":1.5}"#).expect("probe mutation decodes");
    let identity = apply(&before(), &degenerate);
    assert_eq!(identity, before(), "rotate-assets/spins-asset-hero-about-z: a zero-length axis composes the identity quaternion, leaving the orientation as it was");
    assert!(degenerate.diff(&before()).messages().is_empty(), "rotate-assets/spins-asset-hero-about-z: the degenerate axis is a silent fallback, not a diagnostic");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the delta carries the COMPOSED quaternion, not the axis-angle the payload named —
/// the multiplication happens in the diff builder, never at apply time.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rotate-assets/spins-asset-hero-about-z: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["assets"]["patched"][0]["patch"]["orientation"][3], (1.5f64 * 0.5).cos(), "rotate-assets/spins-asset-hero-about-z: the stored w is cos(angle/2), i.e. already composed");
    assert!(committed["assets"]["patched"][0]["patch"]["origin"].is_null(), "rotate-assets/spins-asset-hero-about-z: a rotation fills only the `orientation` slot");
    assert_eq!(committed["assets"]["patched"].as_array().expect("patched is an array").len(), 1, "rotate-assets/spins-asset-hero-about-z: only the addressed asset gets an entry");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed quaternion patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rotate-assets/spins-asset-hero-about-z: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single orientation patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rotate-assets/spins-asset-hero-about-z: committed diff did not carry before to after");
}
