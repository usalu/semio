//! 🔺 Diff constructor for `ChangeAssetUrl`. Error `target-missing` when absent, Warning `no-op`
//! when already at that url.

use super::ChangeAssetUrl;
use crate::diff::ShootingDiff;
use crate::ShootingAssetPatch;
use crate::ShootingSnapshot;

pub fn diff(payload: &ChangeAssetUrl, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.assets.iter().find(|asset| asset.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.url == payload.new_url {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Asset \"{}\" already has that url.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff::asset_patches([(payload.id.clone(), ShootingAssetPatch { url: Some(payload.new_url.clone()), ..Default::default() })]))
}
