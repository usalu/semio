//! 🔺 Diff constructor for `ReorderAssets`. Error `target-missing` when absent, Warning `no-op`
//! when the resulting order is unchanged.

use super::mutation::ReorderAssets;
use crate::artifacts::shooting::diff::{ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &ReorderAssets, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !base.assets.iter().any(|asset| asset.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let original: Vec<String> = base.assets.iter().map(|asset| asset.id.clone()).collect();
    let mut ids = original.clone();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    if ids == original {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Asset \"{}\" order is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff { assets: Some(ShootingAssetsDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() })
}
