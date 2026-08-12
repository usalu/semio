//! 🔺 Diff constructor for `ReplaceSavedCameraView`.

use super::mutation::ReplaceSavedCameraView;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingSavedCameraPatchEntry, ShootingSavedCamerasDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingSavedCameraPatch;

pub fn diff(payload: &ReplaceSavedCameraView, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        saved_cameras: Some(ShootingSavedCamerasDelta {
            patched: vec![ShootingSavedCameraPatchEntry { id: payload.id.clone(), patch: ShootingSavedCameraPatch { label: None, camera: Some(payload.new_camera.clone()) } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
