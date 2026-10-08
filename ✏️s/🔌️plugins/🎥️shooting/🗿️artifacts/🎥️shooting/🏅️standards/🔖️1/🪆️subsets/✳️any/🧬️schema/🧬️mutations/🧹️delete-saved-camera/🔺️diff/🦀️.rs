//! 🔺 Diff constructor for `DeleteSavedCamera`. Error `target-missing` when absent.

use super::DeleteSavedCamera;
use crate::diff::{ShootingDiff, ShootingEdit};
use crate::ShootingSnapshot;

pub fn diff(payload: &DeleteSavedCamera, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !base.saved_cameras.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Saved camera \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(ShootingDiff::camera_edit(ShootingEdit::Remove { id: payload.id.clone() }))
}
