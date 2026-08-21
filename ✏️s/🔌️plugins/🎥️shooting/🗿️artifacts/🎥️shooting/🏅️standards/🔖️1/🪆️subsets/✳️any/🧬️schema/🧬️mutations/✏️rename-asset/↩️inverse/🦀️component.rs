//! ↩ Inverse constructor for `RenameAsset` — reconstructed from BASE state.

use super::mutation::RenameAsset;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &RenameAsset, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.assets.iter().find(|asset| asset.id == payload.id) {
        Some(asset) => vec![ShootingMutation::RenameAsset(RenameAsset { id: payload.id.clone(), new_name: asset.name.clone() })],
        None => Vec::new(),
    }
}
