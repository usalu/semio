//! ↩ Inverse constructor for `ReplaceShotCamera` — reconstructed from BASE state.

use super::ReplaceShotCamera;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &ReplaceShotCamera, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    let camera_id = match base.shots.iter().find(|shot| shot.id == payload.shot_id).and_then(|shot| shot.camera_id.clone()) {
        Some(id) => id,
        None => return Vec::new(),
    };
    match base.saved_cameras.iter().find(|entry| entry.id == camera_id) {
        Some(entry) => vec![ShootingMutation::ReplaceShotCamera(ReplaceShotCamera { shot_id: payload.shot_id.clone(), new_camera: entry.camera.clone() })],
        None => Vec::new(),
    }
}
