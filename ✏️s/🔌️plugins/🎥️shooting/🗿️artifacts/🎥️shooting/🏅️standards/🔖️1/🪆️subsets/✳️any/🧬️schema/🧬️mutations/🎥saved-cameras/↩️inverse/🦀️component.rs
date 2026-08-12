//! ↩ Inverse constructors for the `savedCameras` collection's mutation kinds — reconstructed from
//! captured BASE state. Missing target ⇒ `Vec::new()`.

use super::mutation::{CreateSavedCamera, DeleteSavedCamera, RenameSavedCamera, ReorderSavedCameras, ReplaceSavedCameraView};
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 🌱️CreateSavedCamera
pub fn inverse_create_saved_camera(payload: &CreateSavedCamera, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DeleteSavedCamera(DeleteSavedCamera { id: payload.saved_camera.id.clone() })]
}
//#endregion 🌱️CreateSavedCamera

//#region 🗑️DeleteSavedCamera
pub fn inverse_delete_saved_camera(payload: &DeleteSavedCamera, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.saved_cameras.iter().position(|entry| entry.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateSavedCamera(CreateSavedCamera { saved_camera: base.saved_cameras[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeleteSavedCamera

//#region ✏️RenameSavedCamera
pub fn inverse_rename_saved_camera(payload: &RenameSavedCamera, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.saved_cameras.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![ShootingMutation::RenameSavedCamera(RenameSavedCamera { id: payload.id.clone(), new_label: entry.label.clone() })],
        None => Vec::new(),
    }
}
//#endregion ✏️RenameSavedCamera

//#region 🎥️ReplaceSavedCameraView
pub fn inverse_replace_saved_camera_view(payload: &ReplaceSavedCameraView, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.saved_cameras.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![ShootingMutation::ReplaceSavedCameraView(ReplaceSavedCameraView { id: payload.id.clone(), new_camera: entry.camera.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🎥️ReplaceSavedCameraView

//#region 🔀️ReorderSavedCameras
pub fn inverse_reorder_saved_cameras(payload: &ReorderSavedCameras, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.saved_cameras.iter().position(|entry| entry.id == payload.id) {
        Some(original_index) => vec![ShootingMutation::ReorderSavedCameras(ReorderSavedCameras { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }
}
//#endregion 🔀️ReorderSavedCameras
