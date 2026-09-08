//! 🔺 Diff constructor for `DeleteAsset`. Error `target-missing` when absent.

use super::DeleteAsset;
use crate::diff::{ShootingAssetsDelta, ShootingDiff};
use crate::ShootingSnapshot;

pub fn diff(payload: &DeleteAsset, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !base.assets.iter().any(|asset| asset.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(ShootingDiff { assets: Some(ShootingAssetsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
