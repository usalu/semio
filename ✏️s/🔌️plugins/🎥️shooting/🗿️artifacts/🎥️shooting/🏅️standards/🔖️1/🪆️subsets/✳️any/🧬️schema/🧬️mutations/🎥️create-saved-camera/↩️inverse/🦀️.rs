//! ↩ Inverse constructor for `CreateSavedCamera` — reconstructed from BASE state.

use super::CreateSavedCamera;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &CreateSavedCamera, _base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ShootingMutation::DeleteSavedCamera(crate::mutations::delete_saved_camera::DeleteSavedCamera { id: payload.saved_camera.id.clone() })]

    })())
}
