//! 🔺 Diff constructor for `ReorderAssets`. Error `target-missing` when absent, Warning `no-op`
//! when the resulting order is unchanged.

use super::ReorderAssets;
use crate::diff::{ShootingDiff, ShootingEdit};
use crate::ShootingSnapshot;

pub fn diff(payload: &ReorderAssets, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(from) = base.assets.iter().position(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let index = payload.to_index.min(base.assets.len() - 1);
    if index == from {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Asset \"{}\" order is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff::asset_edit(ShootingEdit::Move { id: payload.id.clone(), index }))
}
