//! 🔺 Diff constructor for `CreateSavedCamera`. Fatal `duplicate-id` on an existing id.

use super::CreateSavedCamera;
use crate::diff::{ShootingDiff, ShootingEdit};
use crate::ShootingSnapshot;

pub fn diff(payload: &CreateSavedCamera, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.saved_cameras.iter().any(|entry| entry.id == payload.saved_camera.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A saved camera with id \"{}\" already exists.", payload.saved_camera.id), [payload.saved_camera.id.clone()]);
    }
    let index = payload.index.map_or(base.saved_cameras.len(), |index| index.min(base.saved_cameras.len()));
    protocol::MutationOutcome::new(ShootingDiff::camera_edit(ShootingEdit::Add { index, item: payload.saved_camera.clone() }))
}
