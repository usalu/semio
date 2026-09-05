//! ↩ Inverse constructor for `ChangeAssetUrl` — reconstructed from BASE state.

use super::ChangeAssetUrl;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &ChangeAssetUrl, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.assets.iter().find(|asset| asset.id == payload.id) {
        Some(asset) => vec![ShootingMutation::ChangeAssetUrl(ChangeAssetUrl { id: payload.id.clone(), new_url: asset.url.clone() })],
        None => Vec::new(),
    }
}
