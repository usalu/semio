//! ↩ Inverse constructor for `RenameAsset` — reconstructed from BASE state.

use super::RenameAsset;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &RenameAsset, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.assets.iter().find(|asset| asset.id == payload.id) {
        Some(asset) => vec![ShootingMutation::RenameAsset(RenameAsset { id: payload.id.clone(), new_name: asset.name.clone() })],
        None => Vec::new(),
    }

    })())
}
