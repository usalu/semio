//! 🔺 Diff constructors for the `savedCameras` collection's mutation kinds — each builds
//! [`ShootingDiff`] sparsely and directly from its payload.

use super::mutation::{CreateSavedCamera, DeleteSavedCamera, RenameSavedCamera, ReorderSavedCameras, ReplaceSavedCameraView};
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingSavedCameraPatchEntry, ShootingSavedCamerasDelta};
use crate::artifacts::shooting::{ShootingSavedCameraPatch, ShootingSnapshot};

//#region 🌱️CreateSavedCamera
pub fn diff_create_saved_camera(payload: &CreateSavedCamera, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { saved_cameras: Some(ShootingSavedCamerasDelta { added: vec![payload.saved_camera.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🌱️CreateSavedCamera

//#region 🗑️DeleteSavedCamera
pub fn diff_delete_saved_camera(payload: &DeleteSavedCamera, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { saved_cameras: Some(ShootingSavedCamerasDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🗑️DeleteSavedCamera

//#region ✏️RenameSavedCamera
pub fn diff_rename_saved_camera(payload: &RenameSavedCamera, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        saved_cameras: Some(ShootingSavedCamerasDelta {
            patched: vec![ShootingSavedCameraPatchEntry { id: payload.id.clone(), patch: ShootingSavedCameraPatch { label: Some(payload.new_label.clone()), camera: None } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion ✏️RenameSavedCamera

//#region 🎥️ReplaceSavedCameraView
pub fn diff_replace_saved_camera_view(payload: &ReplaceSavedCameraView, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        saved_cameras: Some(ShootingSavedCamerasDelta {
            patched: vec![ShootingSavedCameraPatchEntry { id: payload.id.clone(), patch: ShootingSavedCameraPatch { label: None, camera: Some(payload.new_camera.clone()) } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🎥️ReplaceSavedCameraView

//#region 🔀️ReorderSavedCameras
pub fn diff_reorder_saved_cameras(payload: &ReorderSavedCameras, base: &ShootingSnapshot) -> ShootingDiff {
    let mut ids: Vec<String> = base.saved_cameras.iter().map(|entry| entry.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    ShootingDiff { saved_cameras: Some(ShootingSavedCamerasDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
//#endregion 🔀️ReorderSavedCameras
