//! ↩ Inverse constructor for `DeleteSavedCamera` — reconstructed from BASE state.

use super::DeleteSavedCamera;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &DeleteSavedCamera, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_saved_camera::CreateSavedCamera>,
{
    Ok(match base.saved_cameras.iter().position(|entry| entry.id == payload.id) {
        Some(index) => vec![Op::from(crate::mutations::create_saved_camera::CreateSavedCamera { saved_camera: base.saved_cameras[index].clone(), index: Some(index) })],
        None => Vec::new(),
    })
}
