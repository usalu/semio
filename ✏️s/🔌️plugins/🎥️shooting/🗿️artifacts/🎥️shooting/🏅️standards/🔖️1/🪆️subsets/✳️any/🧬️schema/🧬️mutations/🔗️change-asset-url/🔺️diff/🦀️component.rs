//! 🔺 Diff constructor for `ChangeAssetUrl`. Error `target-missing` when absent, Warning `no-op`
//! when already at that url.

use super::mutation::ChangeAssetUrl;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingAssetPatch;

pub async fn diff(payload: &ChangeAssetUrl, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.assets.iter().find(|asset| asset.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.url == payload.new_url {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Asset \"{}\" already has that url.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff {
        assets: Some(ShootingAssetsDelta {
            patched: vec![ShootingAssetPatchEntry { id: payload.id.clone(), patch: ShootingAssetPatch { url: Some(payload.new_url.clone()), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
