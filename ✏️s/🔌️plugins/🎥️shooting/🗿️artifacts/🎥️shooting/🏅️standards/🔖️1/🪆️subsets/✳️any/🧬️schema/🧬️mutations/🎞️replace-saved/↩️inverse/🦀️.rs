//! ↩ Inverse constructor for `ReplaceSavedCameraView` — reconstructed from BASE state.

use super::ReplaceSavedCameraView;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &ReplaceSavedCameraView, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.saved_cameras.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![ShootingMutation::ReplaceSavedCameraView(ReplaceSavedCameraView { id: payload.id.clone(), new_camera: entry.camera.clone() })],
        None => Vec::new(),
    }

    })())
}
