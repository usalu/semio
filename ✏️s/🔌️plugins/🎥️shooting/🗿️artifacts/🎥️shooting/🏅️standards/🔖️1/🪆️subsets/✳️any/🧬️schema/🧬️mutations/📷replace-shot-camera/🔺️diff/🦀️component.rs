//! 🔺 Diff constructor for `ReplaceShotCamera`.

use super::mutation::ReplaceShotCamera;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingSavedCameraPatchEntry, ShootingSavedCamerasDelta};
use crate::artifacts::shooting::ShootingSavedCameraPatch;

pub fn diff(payload: &ReplaceShotCamera, base: &ShootingSnapshot) -> ShootingDiff {
    let camera_id = match base.shots.iter().find(|shot| shot.id == payload.shot_id).and_then(|shot| shot.camera_id.clone()) {
        Some(id) => id,
        None => return ShootingDiff::default(),
    };
    ShootingDiff {
        saved_cameras: Some(ShootingSavedCamerasDelta {
            patched: vec![ShootingSavedCameraPatchEntry { id: camera_id, patch: ShootingSavedCameraPatch { label: None, camera: Some(payload.new_camera.clone()) } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
