//! ↩ Inverse constructor for `DeleteSavedCamera` — reconstructed from BASE state.

use super::DeleteSavedCamera;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &DeleteSavedCamera, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.saved_cameras.iter().position(|entry| entry.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateSavedCamera(crate::mutations::create_saved_camera::CreateSavedCamera { saved_camera: base.saved_cameras[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
