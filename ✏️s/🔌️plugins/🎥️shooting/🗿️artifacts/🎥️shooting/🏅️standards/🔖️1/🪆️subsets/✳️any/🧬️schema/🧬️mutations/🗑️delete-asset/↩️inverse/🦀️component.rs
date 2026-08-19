//! ↩ Inverse constructor for `DeleteAsset` — reconstructed from BASE state.

use super::mutation::DeleteAsset;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub async fn inverse(payload: &DeleteAsset, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.assets.iter().position(|asset| asset.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateAsset(crate::artifacts::shooting::mutations::create_asset::mutation::CreateAsset { asset: base.assets[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
