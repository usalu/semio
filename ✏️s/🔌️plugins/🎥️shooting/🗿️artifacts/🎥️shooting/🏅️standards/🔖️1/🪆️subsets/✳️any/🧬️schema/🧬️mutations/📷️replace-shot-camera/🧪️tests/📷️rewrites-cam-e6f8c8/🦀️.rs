//! 🧪️ `replace-shot-camera` fixture — `📷️rewrites-cam-wide-through-shot-wide`.
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
    step.diff(base).into_parts().0.apply(base).expect("replace-shot-camera diff applies")
}

/// ▶️ `replace-shot-camera` is addressed by SHOT id but writes through to the `savedCameras` entry
/// that shot points at — the shot record itself is never part of the diff, only its camera's pose.
#[semio_framework_async_macros::async_test]
async fn writes_through_the_shot_into_its_saved_camera() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "replace-shot-camera/rewrites-cam-wide-through-shot-wide: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.shots, before().shots, "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the shot record is never patched, only dereferenced");
    assert_eq!(snapshot.saved_cameras[0].camera.position, [3.0, -3.0, 2.0], "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the pose lands on \"cam-wide\", the camera \"shot-wide\" references");
    assert_eq!(snapshot.saved_cameras[0].label, before().saved_cameras[0].label, "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the camera's label is left alone by this patch");
    assert_eq!(snapshot.saved_cameras[1], before().saved_cameras[1], "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the unreferenced camera is untouched");
}

/// ↩️ The inverse re-derives the referenced camera from BASE and writes its old pose back.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_pose() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "replace-shot-camera/rewrites-cam-wide-through-shot-wide: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-shot-camera/rewrites-cam-wide-through-shot-wide: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-shot-camera/rewrites-cam-wide-through-shot-wide: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and this leaf's own dereference guard: a shot with
/// NO `cameraId` has nothing to write through to, so it reports `mutation.no-op` at Warning rather
/// than minting a saved camera.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_an_unbound_shot_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-shot-camera/rewrites-cam-wide-through-shot-wide: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "replace-shot-camera/rewrites-cam-wide-through-shot-wide: a bound shot must raise no diagnostic");

    let unbound: ShootingMutation = serde_json::from_str(r#"{"mutation":"replaceShotCamera","shot_id":"shot-close","new_camera":{"position":[3.0,-3.0,2.0],"target":[0.0,0.0,0.5],"zoom":1.5,"fov":40.0}}"#).expect("probe mutation decodes");
    let skipped = unbound.diff(&before());
    assert_eq!(skipped.worst_level(), Some(protocol::Severity::Warning), "replace-shot-camera/rewrites-cam-wide-through-shot-wide: an unbound shot is a Warning, not an Error");
    assert_eq!(skipped.messages()[0].code.0, "mutation.no-op", "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the dereference guard's frozen code");
    let unchanged = skipped.into_parts().0.apply(&before()).expect("a no-op outcome still applies");
    assert_eq!(unchanged.saved_cameras, before().saved_cameras, "replace-shot-camera/rewrites-cam-wide-through-shot-wide: an unbound shot must not mint a saved camera");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it is the only place the write-through is visible: the payload names a SHOT, the delta patches a
/// SAVED CAMERA, and `shots` is left NULL entirely.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-shot-camera/rewrites-cam-wide-through-shot-wide: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["savedCameras"]["patched"][0]["id"], "cam-wide", "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the delta is keyed by the dereferenced CAMERA id, not the payload's shot id");
    assert!(committed["shots"].is_null(), "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the `shots` collection is not opened at all");
    assert!(committed["savedCameras"]["patched"][0]["patch"]["label"].is_null(), "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the patch fills `camera` and leaves `label` null");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed write-through patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-shot-camera/rewrites-cam-wide-through-shot-wide: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the saved-camera patch alone is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-shot-camera/rewrites-cam-wide-through-shot-wide: committed diff did not carry before to after");
}
