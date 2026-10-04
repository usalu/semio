//! ↩ Inverse constructor for `ChangeAssetUrl` — reconstructed from BASE state.

use super::ChangeAssetUrl;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &ChangeAssetUrl, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.assets.iter().find(|asset| asset.id == payload.id) {
        Some(asset) => vec![ShootingMutation::ChangeAssetUrl(ChangeAssetUrl { id: payload.id.clone(), new_url: asset.url.clone() })],
        None => Vec::new(),
    }

    })())
}
