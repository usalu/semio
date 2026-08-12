//! 🔺 Diff constructor for `ReplaceShotCamera` — resolves the saved-camera entry `shot_id`
//! references and patches only that entry's `camera` field.

use super::mutation::ReplaceShotCamera;
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingSavedCameraPatchEntry, ShootingSavedCamerasDelta};
use crate::artifacts::shooting::{ShootingSavedCameraPatch, ShootingSnapshot};

//#region 🎯️Resolve
/// 🎯️ Resolves which `savedCameras` entry (if any) `shot_id` targets.
fn resolve_camera_id(base: &ShootingSnapshot, shot_id: &str) -> Option<String> {
    base.shots.iter().find(|shot| shot.id == shot_id).and_then(|shot| shot.camera_id.clone())
}
//#endregion 🎯️Resolve

//#region 📷️ReplaceShotCamera
pub fn diff_replace_shot_camera(payload: &ReplaceShotCamera, base: &ShootingSnapshot) -> ShootingDiff {
    match resolve_camera_id(base, &payload.shot_id) {
        Some(camera_id) => ShootingDiff {
            saved_cameras: Some(ShootingSavedCamerasDelta {
                patched: vec![ShootingSavedCameraPatchEntry { id: camera_id, patch: ShootingSavedCameraPatch { label: None, camera: Some(payload.new_camera.clone()) } }],
                ..Default::default()
            }),
            ..Default::default()
        },
        None => ShootingDiff::default(),
    }
}
//#endregion 📷️ReplaceShotCamera
