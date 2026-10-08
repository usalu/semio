//! 🔺 Diff constructor for `CreateShot`. Fatal `duplicate-id` on an existing id.

use super::CreateShot;
use crate::diff::{ShootingDiff, ShootingEdit};
use crate::ShootingSnapshot;

pub fn diff(payload: &CreateShot, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.shots.iter().any(|entry| entry.id == payload.shot.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A shot with id \"{}\" already exists.", payload.shot.id), [payload.shot.id.clone()]);
    }
    let index = payload.index.map_or(base.shots.len(), |index| index.min(base.shots.len()));
    protocol::MutationOutcome::new(ShootingDiff::shot_edit(ShootingEdit::Add { index, item: payload.shot.clone() }))
}
