//! ↩ Inverse constructor for `ReorderSavedCameras` — reconstructed from BASE state.

use super::ReorderSavedCameras;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &ReorderSavedCameras, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.saved_cameras.iter().position(|entry| entry.id == payload.id) {
        Some(original_index) => vec![ShootingMutation::ReorderSavedCameras(ReorderSavedCameras { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }
}
