//! 🔺 Diff constructor for `ReplaceSavedCameraView`. Error `target-missing` when absent, Warning
//! `no-op` when the pose is unchanged.

use super::mutation::ReplaceSavedCameraView;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingSavedCameraPatchEntry, ShootingSavedCamerasDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingSavedCameraPatch;

pub async fn diff(payload: &ReplaceSavedCameraView, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.saved_cameras.iter().find(|camera| camera.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Saved camera \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.camera == payload.new_camera {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Saved camera \"{}\" view is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff {
        saved_cameras: Some(ShootingSavedCamerasDelta {
            patched: vec![ShootingSavedCameraPatchEntry { id: payload.id.clone(), patch: ShootingSavedCameraPatch { label: None, camera: Some(payload.new_camera.clone()) } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
