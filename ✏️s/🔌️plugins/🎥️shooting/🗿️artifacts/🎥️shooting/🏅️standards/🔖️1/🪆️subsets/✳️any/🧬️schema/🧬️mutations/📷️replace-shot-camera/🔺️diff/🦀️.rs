//! 🔺 Diff constructor for `ReplaceShotCamera`. Error `target-missing` when the shot is absent,
//! Warning `no-op` when that shot has no saved camera.

use super::ReplaceShotCamera;
use crate::diff::ShootingDiff;
use crate::ShootingSavedCameraPatch;
use crate::ShootingSnapshot;

pub fn diff(payload: &ReplaceShotCamera, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(shot) = base.shots.iter().find(|shot| shot.id == payload.shot_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.shot_id), [payload.shot_id.clone()]);
    };
    let Some(camera_id) = shot.camera_id.clone() else {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Shot \"{}\" has no saved camera to replace.", payload.shot_id));
    };
    let Some(saved) = base.saved_cameras.iter().find(|entry| entry.id == camera_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Saved camera \"{}\" of shot \"{}\" does not exist.", camera_id, payload.shot_id), [camera_id.clone()]);
    };
    if saved.camera == payload.new_camera {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Saved camera \"{}\" view is unchanged.", camera_id));
    }
    protocol::MutationOutcome::new(ShootingDiff::camera_patches([(camera_id, ShootingSavedCameraPatch { label: None, camera: Some(payload.new_camera.clone()) })]))
}
