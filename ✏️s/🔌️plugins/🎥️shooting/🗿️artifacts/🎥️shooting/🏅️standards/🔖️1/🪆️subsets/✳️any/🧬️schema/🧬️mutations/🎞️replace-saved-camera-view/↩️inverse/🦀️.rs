//! ↩ Inverse constructor for `ReplaceSavedCameraView` — reconstructed from BASE state.

use super::ReplaceSavedCameraView;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ReplaceSavedCameraView, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::ReplaceSavedCameraView>,
{
    Ok((|| {
    match base.saved_cameras.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![Op::from(ReplaceSavedCameraView { id: payload.id.clone(), new_camera: entry.camera.clone() })],
        None => Vec::new(),
    }

    })())
}
