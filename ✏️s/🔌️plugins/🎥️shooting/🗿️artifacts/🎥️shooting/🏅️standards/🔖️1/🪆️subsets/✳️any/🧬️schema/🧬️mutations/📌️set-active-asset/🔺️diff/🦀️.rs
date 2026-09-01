//! 🔺 Diff constructor for `SetActiveAsset`. Error `target-missing` when addressing an unknown
//! asset, Warning `no-op` when already active.

use super::SetActiveAsset;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &SetActiveAsset, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let next = payload.asset_id.clone().unwrap_or_default();
    if let Some(id) = &payload.asset_id {
        if !base.assets.iter().any(|asset| &asset.id == id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", id), [id.clone()]);
        }
    }
    if base.active_asset_id == next {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Active asset is unchanged.");
    }
    protocol::MutationOutcome::new(ShootingDiff { active_asset_id: Some(next), ..Default::default() })
}
