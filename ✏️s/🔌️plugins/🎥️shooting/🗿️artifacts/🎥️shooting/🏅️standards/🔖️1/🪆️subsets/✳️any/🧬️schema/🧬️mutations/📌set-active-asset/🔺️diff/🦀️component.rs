//! 🔺 Diff constructor for `SetActiveAsset`.

use super::mutation::SetActiveAsset;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &SetActiveAsset, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { active_asset_id: Some(payload.asset_id.clone().unwrap_or_default()), ..Default::default() }
}
