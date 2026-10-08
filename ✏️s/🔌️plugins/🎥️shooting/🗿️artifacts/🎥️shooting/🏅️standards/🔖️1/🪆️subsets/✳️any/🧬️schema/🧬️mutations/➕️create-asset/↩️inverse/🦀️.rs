//! ↩ Inverse constructor for `CreateAsset` — reconstructed from BASE state.

use super::CreateAsset;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &CreateAsset, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok(match base.assets.iter().any(|entry| entry.id == payload.asset.id) {
        true => Vec::new(),
        false => vec![ShootingMutation::DeleteAsset(crate::mutations::delete_asset::DeleteAsset { id: payload.asset.id.clone() })],
    })
}
