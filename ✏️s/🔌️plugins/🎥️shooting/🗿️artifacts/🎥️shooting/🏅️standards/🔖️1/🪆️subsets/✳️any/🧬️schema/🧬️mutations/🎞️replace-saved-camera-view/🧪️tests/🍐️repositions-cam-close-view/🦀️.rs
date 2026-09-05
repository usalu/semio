//! 🧪️ `replace-saved-camera-view` fixture — `🍐️repositions-cam-close-view`.
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
    step.diff(base).into_parts().0.apply(base).expect("replace-saved-camera-view diff applies")
}

/// ▶️ `replace-saved-camera-view` is addressed by CAMERA id (unlike `replace-shot-camera`) and
/// REPLACES the whole pose record — `position`, `target`, `zoom` and `fov` all come from the
/// payload, none of them are merged with what was there.
#[semio_framework_async_macros::async_test]
async fn replaces_the_whole_pose_of_the_addressed_camera() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "replace-saved-camera-view/repositions-cam-close-view: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.saved_cameras[1].camera.position, [1.0, -1.0, 0.75], "replace-saved-camera-view/repositions-cam-close-view: the new eye point");
    assert_eq!(snapshot.saved_cameras[1].camera.zoom, 4.0, "replace-saved-camera-view/repositions-cam-close-view: `zoom` is replaced, not preserved");
    assert_eq!(snapshot.saved_cameras[1].camera.fov, 20.0, "replace-saved-camera-view/repositions-cam-close-view: `fov` is replaced, not preserved");
    assert_eq!(snapshot.saved_cameras[1].label, before().saved_cameras[1].label, "replace-saved-camera-view/repositions-cam-close-view: the patch's `label: None` leaves the caption untouched");
    assert_eq!(snapshot.shots, before().shots, "replace-saved-camera-view/repositions-cam-close-view: no shot record is rewritten");
}

/// ↩️ The inverse is a `replace-saved-camera-view` carrying the BASE pose back.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_pose() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "replace-saved-camera-view/repositions-cam-close-view: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-saved-camera-view/repositions-cam-close-view: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-saved-camera-view/repositions-cam-close-view: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the pose-equality guard: replaying the same pose
/// compares the WHOLE `ShootingCamera` and reports `mutation.no-op` at Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_an_identical_pose_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-saved-camera-view/repositions-cam-close-view: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "replace-saved-camera-view/repositions-cam-close-view: a real reposition must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "replace-saved-camera-view/repositions-cam-close-view: an identical pose is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "replace-saved-camera-view/repositions-cam-close-view: the pose-equality guard's frozen code");

    let ghost: ShootingMutation = serde_json::from_str(r#"{"mutation":"replaceSavedCameraView","id":"cam-ghost","new_camera":{"position":[1.0,-1.0,0.75],"target":[0.0,0.0,1.0],"zoom":4.0,"fov":20.0}}"#).expect("probe mutation decodes");
    let rejected = ghost.diff(&before());
    assert_eq!(rejected.messages()[0].code.0, "mutation.target-missing", "replace-saved-camera-view/repositions-cam-close-view: an unknown camera id is target-missing, not a no-op");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the mirror of its rename sibling: `{label: None, camera: Some}` — and that the camera
/// slot carries the WHOLE pose, so nothing is merged with the old one.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-saved-camera-view/repositions-cam-close-view: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["savedCameras"]["patched"][0]["patch"]["camera"]["zoom"], 4.0, "replace-saved-camera-view/repositions-cam-close-view: the whole replacement pose is in the delta");
    assert!(committed["savedCameras"]["patched"][0]["patch"]["label"].is_null(), "replace-saved-camera-view/repositions-cam-close-view: the `label` slot is explicitly null in the delta");
    assert_eq!(committed["savedCameras"]["patched"][0]["id"], "cam-close", "replace-saved-camera-view/repositions-cam-close-view: the delta is keyed by the payload's own camera id");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed pose replacement round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-saved-camera-view/repositions-cam-close-view: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single camera patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-saved-camera-view/repositions-cam-close-view: committed diff did not carry before to after");
}
