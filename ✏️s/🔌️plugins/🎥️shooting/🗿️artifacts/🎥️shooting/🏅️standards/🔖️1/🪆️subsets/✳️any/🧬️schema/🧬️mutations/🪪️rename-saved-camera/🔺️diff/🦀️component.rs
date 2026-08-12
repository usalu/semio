//! 🔺 Diff constructor for `RenameSavedCamera`.

use super::mutation::RenameSavedCamera;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingSavedCameraPatchEntry, ShootingSavedCamerasDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingSavedCameraPatch;

pub fn diff(payload: &RenameSavedCamera, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        saved_cameras: Some(ShootingSavedCamerasDelta {
            patched: vec![ShootingSavedCameraPatchEntry { id: payload.id.clone(), patch: ShootingSavedCameraPatch { label: Some(payload.new_label.clone()), camera: None } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
