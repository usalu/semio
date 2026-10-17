//! ↩ Inverse constructor for `SetActiveAsset` — reconstructed from BASE state.

use super::SetActiveAsset;
use crate::ShootingSnapshot;

pub fn inverse<Op>(_payload: &SetActiveAsset, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_active_asset::SetActiveAsset>,
{
    Ok((|| {
    let asset_id = if base.active_asset_id.is_empty() { None } else { Some(base.active_asset_id.clone()) };
    vec![Op::from(SetActiveAsset { asset_id })]

    })())
}
