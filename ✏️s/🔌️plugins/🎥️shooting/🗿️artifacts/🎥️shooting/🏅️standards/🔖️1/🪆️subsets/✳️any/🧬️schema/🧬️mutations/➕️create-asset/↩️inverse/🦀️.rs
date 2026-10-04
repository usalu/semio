//! ↩ Inverse constructor for `CreateAsset` — reconstructed from BASE state.

use super::CreateAsset;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &CreateAsset, _base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ShootingMutation::DeleteAsset(crate::mutations::delete_asset::DeleteAsset { id: payload.asset.id.clone() })]

    })())
}
