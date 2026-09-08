//! ↩ Inverse constructor for `DeleteAsset` — reconstructed from BASE state.

use super::DeleteAsset;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &DeleteAsset, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.assets.iter().position(|asset| asset.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateAsset(crate::mutations::create_asset::CreateAsset { asset: base.assets[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
