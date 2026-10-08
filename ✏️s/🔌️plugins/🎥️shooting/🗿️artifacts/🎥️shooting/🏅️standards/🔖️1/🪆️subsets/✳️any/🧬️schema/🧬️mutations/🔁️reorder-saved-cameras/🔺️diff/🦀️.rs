//! 🔺 Diff constructor for `ReorderSavedCameras`. Error `target-missing` when absent, Warning
//! `no-op` when the resulting order is unchanged.

use super::ReorderSavedCameras;
use crate::diff::{ShootingDiff, ShootingEdit};
use crate::ShootingSnapshot;

pub fn diff(payload: &ReorderSavedCameras, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(from) = base.saved_cameras.iter().position(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Saved camera \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let index = payload.to_index.min(base.saved_cameras.len() - 1);
    if index == from {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Saved camera \"{}\" order is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff::camera_edit(ShootingEdit::Move { id: payload.id.clone(), index }))
}
