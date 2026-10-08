//! 🔺 Diff constructor for `ReorderShots`. Error `target-missing` when absent, Warning `no-op`
//! when the resulting order is unchanged.

use super::ReorderShots;
use crate::diff::{ShootingDiff, ShootingEdit};
use crate::ShootingSnapshot;

pub fn diff(payload: &ReorderShots, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(from) = base.shots.iter().position(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let index = payload.to_index.min(base.shots.len() - 1);
    if index == from {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Shot \"{}\" order is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff::shot_edit(ShootingEdit::Move { id: payload.id.clone(), from, to: index }))
}
