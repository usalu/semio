//! 🔺 Diff constructor for `RenameAsset`. Error `target-missing` when absent, Warning `no-op` when
//! already at that name.

use super::mutation::RenameAsset;
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingAssetPatch;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &RenameAsset, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.assets.iter().find(|asset| asset.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Asset \"{}\" already has name \"{}\".", payload.id, payload.new_name));
    }
    protocol::MutationOutcome::new(ShootingDiff {
        assets: Some(ShootingAssetsDelta { patched: vec![ShootingAssetPatchEntry { id: payload.id.clone(), patch: ShootingAssetPatch { name: Some(payload.new_name.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
