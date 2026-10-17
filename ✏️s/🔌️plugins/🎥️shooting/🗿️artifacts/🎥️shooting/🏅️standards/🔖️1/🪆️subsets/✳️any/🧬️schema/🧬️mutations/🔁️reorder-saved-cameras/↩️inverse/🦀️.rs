//! ↩ Inverse constructor for `ReorderSavedCameras` — reconstructed from BASE state.

use super::ReorderSavedCameras;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ReorderSavedCameras, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::reorder_saved_cameras::ReorderSavedCameras>,
{
    Ok((|| {
    match base.saved_cameras.iter().position(|entry| entry.id == payload.id) {
        Some(original_index) => vec![Op::from(ReorderSavedCameras { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }

    })())
}
