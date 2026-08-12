//! 🔺 Diff constructor for `ReorderAssets`.

use super::mutation::ReorderAssets;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingAssetsDelta, ShootingDiff};

pub fn diff(payload: &ReorderAssets, base: &ShootingSnapshot) -> ShootingDiff {
    let mut ids: Vec<String> = base.assets.iter().map(|asset| asset.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    ShootingDiff { assets: Some(ShootingAssetsDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
