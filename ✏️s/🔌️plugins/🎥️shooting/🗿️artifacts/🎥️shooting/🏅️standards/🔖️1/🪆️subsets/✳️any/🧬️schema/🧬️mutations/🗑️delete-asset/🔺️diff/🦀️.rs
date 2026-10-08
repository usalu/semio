//! 🔺 Diff constructor for `DeleteAsset`. Error `target-missing` when absent.

use super::DeleteAsset;
use crate::diff::{ShootingDiff, ShootingEdit};
use crate::ShootingSnapshot;

pub fn diff(payload: &DeleteAsset, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !base.assets.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(ShootingDiff::asset_edit(ShootingEdit::Remove { id: payload.id.clone() }))
}
