//! 🔺 Diff constructor for `DeleteShot`. Error `target-missing` when absent.

use super::DeleteShot;
use crate::diff::{ShootingDiff, ShootingEdit};
use crate::ShootingSnapshot;

pub fn diff(payload: &DeleteShot, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(index) = base.shots.iter().position(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(ShootingDiff::shot_edit(ShootingEdit::Remove { id: payload.id.clone(), index }))
}
