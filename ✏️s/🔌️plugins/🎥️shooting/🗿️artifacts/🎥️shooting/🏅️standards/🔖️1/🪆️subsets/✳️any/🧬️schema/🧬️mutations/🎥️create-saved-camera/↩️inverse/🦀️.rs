//! ↩ Inverse constructor for `CreateSavedCamera` — reconstructed from BASE state.

use super::CreateSavedCamera;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &CreateSavedCamera, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DeleteSavedCamera(crate::artifacts::shooting::mutations::delete_saved_camera::DeleteSavedCamera { id: payload.saved_camera.id.clone() })]
}
