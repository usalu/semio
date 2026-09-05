//! ↩ Inverse constructor for `CreateAsset` — reconstructed from BASE state.

use super::CreateAsset;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &CreateAsset, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DeleteAsset(crate::artifacts::shooting::mutations::delete_asset::DeleteAsset { id: payload.asset.id.clone() })]
}
