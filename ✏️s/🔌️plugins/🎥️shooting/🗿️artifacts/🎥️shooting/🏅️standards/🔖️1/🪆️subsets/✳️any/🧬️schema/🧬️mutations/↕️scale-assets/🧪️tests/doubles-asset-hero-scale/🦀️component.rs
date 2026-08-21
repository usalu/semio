//! 🧪️ `scale-assets` fixture — `doubles-asset-hero-scale`.
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
    step.diff(base).into_parts().0.apply(base).expect("scale-assets diff applies")
}

/// ▶️ `scale-assets` MULTIPLIES each factor into the asset's existing scale rather than replacing
/// it: "asset-hero" already sits at `[2, 2, 2]`, so doubling lands on `[4, 4, 4]`, not `[2, 2, 2]`.
#[semio_framework_async_macros::async_test]
async fn multiplies_into_the_existing_scale() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "scale-assets/doubles-asset-hero-scale: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.assets[0].scale, Some([4.0, 4.0, 4.0]), "scale-assets/doubles-asset-hero-scale: the factors compound with the base scale");
    assert_eq!(snapshot.assets[0].origin, before().assets[0].origin, "scale-assets/doubles-asset-hero-scale: scaling never moves the origin");
    assert_eq!(snapshot.assets[0].orientation, before().assets[0].orientation, "scale-assets/doubles-asset-hero-scale: scaling never touches orientation");
    assert_eq!(snapshot.assets[1].scale, None, "scale-assets/doubles-asset-hero-scale: the unaddressed asset keeps its absent scale");
}

/// ↩️ The inverse is a `scale-assets` by the reciprocal factors.
#[semio_framework_async_macros::async_test]
async fn inverse_scales_by_the_reciprocals() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "scale-assets/doubles-asset-hero-scale: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "scale-assets/doubles-asset-hero-scale: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "scale-assets/doubles-asset-hero-scale: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the positivity invariant: a zero (or negative)
/// factor is `mutation.invariant` at Fatal, which by LAW 1 carries the default diff.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_a_non_positive_factor_is_fatal() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "scale-assets/doubles-asset-hero-scale: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "scale-assets/doubles-asset-hero-scale: a positive finite scale must raise no diagnostic");

    let collapsing: ShootingMutation = serde_json::from_str(r#"{"mutation":"scaleAssets","asset_ids":["asset-hero"],"sx":0.0,"sy":1.0,"sz":1.0}"#).expect("probe mutation decodes");
    let rejected = collapsing.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Fatal), "scale-assets/doubles-asset-hero-scale: collapsing an axis to zero must be Fatal");
    assert_eq!(rejected.messages()[0].code.0, "mutation.invariant", "scale-assets/doubles-asset-hero-scale: the positivity guard's frozen code");
    assert_eq!(rejected.messages()[0].target, vec!["asset-hero".to_string()], "scale-assets/doubles-asset-hero-scale: the invariant is reported against the whole addressed selection");
    let unchanged = rejected.into_parts().0.apply(&before()).expect("a Fatal outcome carries the default diff");
    assert_eq!(unchanged, before(), "scale-assets/doubles-asset-hero-scale: a Fatal scale must leave the snapshot untouched");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the delta carries the PRODUCT `[4, 4, 4]`, not the factors `[2, 2, 2]` — the
/// multiplication is resolved against the base inside the diff builder.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "scale-assets/doubles-asset-hero-scale: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["assets"]["patched"][0]["patch"]["scale"][0], 4.0, "scale-assets/doubles-asset-hero-scale: the delta stores the resolved product, not the factor");
    assert!(committed["assets"]["patched"][0]["patch"]["origin"].is_null() && committed["assets"]["patched"][0]["patch"]["orientation"].is_null(), "scale-assets/doubles-asset-hero-scale: a scale fills only the `scale` slot");
    assert!(committed["shots"].is_null() && committed["savedCameras"].is_null(), "scale-assets/doubles-asset-hero-scale: a transform never leaves the `assets` collection");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed scale patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "scale-assets/doubles-asset-hero-scale: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single scale patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "scale-assets/doubles-asset-hero-scale: committed diff did not carry before to after");
}
