//! ↩ Inverse constructor for `DeleteSavedCamera` — reconstructed from BASE state.

use super::mutation::DeleteSavedCamera;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &DeleteSavedCamera, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.saved_cameras.iter().position(|entry| entry.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateSavedCamera(crate::artifacts::shooting::mutations::create_saved_camera::mutation::CreateSavedCamera { saved_camera: base.saved_cameras[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
