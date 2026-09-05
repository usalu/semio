//! 🧪️ `delete-saved-camera` fixture — `🚫️removes-trailing-cam-close`.
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
    step.diff(base).into_parts().0.apply(base).expect("delete-saved-camera diff applies")
}

/// ▶️ `delete-saved-camera` drops the camera record only — the shot→camera reference is one-way and
/// this diff performs NO cascade over `shots.cameraId`.
#[semio_framework_async_macros::async_test]
async fn removes_the_camera_without_cascading_into_shots() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "delete-saved-camera/removes-trailing-cam-close: applied state differs from committed after-snapshot");
    assert!(!snapshot.saved_cameras.iter().any(|camera| camera.id == "cam-close"), "delete-saved-camera/removes-trailing-cam-close: the addressed camera must be gone");
    assert_eq!(snapshot.saved_cameras[..], before().saved_cameras[..1], "delete-saved-camera/removes-trailing-cam-close: the surviving camera keeps its record and place");
    assert_eq!(snapshot.shots, before().shots, "delete-saved-camera/removes-trailing-cam-close: no shot's `cameraId` is rewritten or cleared");
    assert_eq!(snapshot.assets, before().assets, "delete-saved-camera/removes-trailing-cam-close: the asset list is untouched");
}

/// ↩️ The inverse is a `create-saved-camera` rebuilt from the BASE record.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_deleted_saved_camera() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "delete-saved-camera/removes-trailing-cam-close: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-saved-camera/removes-trailing-cam-close: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-saved-camera/removes-trailing-cam-close: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the presence guard: deleting the same camera
/// twice is `mutation.target-missing` at Error.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_second_delete_is_target_missing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-saved-camera/removes-trailing-cam-close: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "delete-saved-camera/removes-trailing-cam-close: deleting a present camera must raise no diagnostic");

    let second = mutation().diff(&expected_after());
    assert_eq!(second.worst_level(), Some(protocol::Severity::Error), "delete-saved-camera/removes-trailing-cam-close: deleting an absent camera is an Error");
    assert_eq!(second.messages()[0].code.0, "mutation.target-missing", "delete-saved-camera/removes-trailing-cam-close: the absence guard's frozen code");
    assert_eq!(second.messages()[0].target, vec!["cam-close".to_string()], "delete-saved-camera/removes-trailing-cam-close: the missing target is named");
    let unchanged = second.into_parts().0.apply(&expected_after()).expect("an Error outcome carries the default diff");
    assert_eq!(unchanged, expected_after(), "delete-saved-camera/removes-trailing-cam-close: a rejected delete must leave the snapshot untouched");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the delete is a bare id in `savedCameras.removed` and that `shots` stays NULL — no
/// dangling `cameraId` is cleared, which is exactly the behaviour to pin down.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-saved-camera/removes-trailing-cam-close: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["savedCameras"]["removed"][0], "cam-close", "delete-saved-camera/removes-trailing-cam-close: a delete is an id, never a record");
    assert!(committed["shots"].is_null(), "delete-saved-camera/removes-trailing-cam-close: no shot's `cameraId` is cleared — the reference is deliberately left one-way");
    assert!(committed["savedCameras"]["patched"].as_array().expect("patched is an array").is_empty(), "delete-saved-camera/removes-trailing-cam-close: nothing is patched on the way out");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed delete-saved-camera delta round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-saved-camera/removes-trailing-cam-close: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the lone removed id is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-saved-camera/removes-trailing-cam-close: committed diff did not carry before to after");
}
