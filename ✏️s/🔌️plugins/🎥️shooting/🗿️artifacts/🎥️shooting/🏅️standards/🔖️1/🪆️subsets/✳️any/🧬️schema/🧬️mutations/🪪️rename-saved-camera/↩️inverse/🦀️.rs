//! ↩ Inverse constructor for `RenameSavedCamera` — reconstructed from BASE state.

use super::RenameSavedCamera;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &RenameSavedCamera, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_saved_camera::RenameSavedCamera>,
{
    Ok((|| {
    match base.saved_cameras.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![Op::from(RenameSavedCamera { id: payload.id.clone(), new_label: entry.label.clone() })],
        None => Vec::new(),
    }

    })())
}
