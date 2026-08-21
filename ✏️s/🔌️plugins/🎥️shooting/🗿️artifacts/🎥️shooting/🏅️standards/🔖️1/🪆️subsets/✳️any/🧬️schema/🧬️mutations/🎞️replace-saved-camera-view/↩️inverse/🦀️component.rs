//! ↩ Inverse constructor for `ReplaceSavedCameraView` — reconstructed from BASE state.

use super::mutation::ReplaceSavedCameraView;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &ReplaceSavedCameraView, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.saved_cameras.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![ShootingMutation::ReplaceSavedCameraView(ReplaceSavedCameraView { id: payload.id.clone(), new_camera: entry.camera.clone() })],
        None => Vec::new(),
    }
}
