//! ↩ Inverse constructor for `ReorderSavedCameras` — reconstructed from BASE state.

use super::ReorderSavedCameras;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &ReorderSavedCameras, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.saved_cameras.iter().position(|entry| entry.id == payload.id) {
        Some(original_index) => vec![ShootingMutation::ReorderSavedCameras(ReorderSavedCameras { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }

    })())
}
