//! ↩ Inverse constructor for `CreateSavedCamera` — reconstructed from BASE state.

use super::CreateSavedCamera;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &CreateSavedCamera, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok(match base.saved_cameras.iter().any(|entry| entry.id == payload.saved_camera.id) {
        true => Vec::new(),
        false => vec![ShootingMutation::DeleteSavedCamera(crate::mutations::delete_saved_camera::DeleteSavedCamera { id: payload.saved_camera.id.clone() })],
    })
}
