//! 🔺 Diff constructor for `CreateAsset`. Fatal `duplicate-id` on an existing id.

use super::CreateAsset;
use crate::diff::{ShootingDiff, ShootingEdit};
use crate::ShootingSnapshot;

pub fn diff(payload: &CreateAsset, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.assets.iter().any(|entry| entry.id == payload.asset.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An asset with id \"{}\" already exists.", payload.asset.id), [payload.asset.id.clone()]);
    }
    let index = payload.index.map_or(base.assets.len(), |index| index.min(base.assets.len()));
    protocol::MutationOutcome::new(ShootingDiff::asset_edit(ShootingEdit::Add { index, item: payload.asset.clone() }))
}
