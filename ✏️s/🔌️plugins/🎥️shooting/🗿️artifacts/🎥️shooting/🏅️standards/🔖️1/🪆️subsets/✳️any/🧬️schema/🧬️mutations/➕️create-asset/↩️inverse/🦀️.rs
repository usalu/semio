//! ↩ Inverse constructor for `CreateAsset` — reconstructed from BASE state.

use super::CreateAsset;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &CreateAsset, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DeleteAsset(crate::mutations::delete_asset::DeleteAsset { id: payload.asset.id.clone() })]
}
