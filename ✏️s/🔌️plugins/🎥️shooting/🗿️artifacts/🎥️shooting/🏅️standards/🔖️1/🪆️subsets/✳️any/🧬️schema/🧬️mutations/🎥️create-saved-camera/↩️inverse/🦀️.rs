//! ↩ Inverse constructor for `CreateSavedCamera` — reconstructed from BASE state.

use super::CreateSavedCamera;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &CreateSavedCamera, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_saved_camera::DeleteSavedCamera>,
{
    Ok(match base.saved_cameras.iter().any(|entry| entry.id == payload.saved_camera.id) {
        true => Vec::new(),
        false => vec![Op::from(crate::mutations::delete_saved_camera::DeleteSavedCamera { id: payload.saved_camera.id.clone() })],
    })
}
