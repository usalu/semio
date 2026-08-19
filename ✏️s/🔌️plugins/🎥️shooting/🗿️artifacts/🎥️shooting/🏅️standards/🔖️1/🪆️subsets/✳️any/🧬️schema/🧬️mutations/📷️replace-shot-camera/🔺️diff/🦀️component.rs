//! 🔺 Diff constructor for `ReplaceShotCamera`. Error `target-missing` when the shot is absent,
//! Warning `no-op` when that shot has no saved camera.

use super::mutation::ReplaceShotCamera;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingSavedCameraPatchEntry, ShootingSavedCamerasDelta};
use crate::artifacts::shooting::ShootingSavedCameraPatch;

pub async fn diff(payload: &ReplaceShotCamera, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(shot) = base.shots.iter().find(|shot| shot.id == payload.shot_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.shot_id), [payload.shot_id.clone()]);
    };
    let Some(camera_id) = shot.camera_id.clone() else {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shot \"{}\" has no saved camera to replace.", payload.shot_id));
    };
    protocol::MutationOutcome::new(ShootingDiff {
        saved_cameras: Some(ShootingSavedCamerasDelta {
            patched: vec![ShootingSavedCameraPatchEntry { id: camera_id, patch: ShootingSavedCameraPatch { label: None, camera: Some(payload.new_camera.clone()) } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
