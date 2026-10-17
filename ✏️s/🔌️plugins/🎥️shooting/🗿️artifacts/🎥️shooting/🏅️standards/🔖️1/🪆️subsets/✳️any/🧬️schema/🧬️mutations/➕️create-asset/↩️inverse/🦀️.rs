//! ↩ Inverse constructor for `CreateAsset` — reconstructed from BASE state.

use super::CreateAsset;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &CreateAsset, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_asset::DeleteAsset>,
{
    Ok(match base.assets.iter().any(|entry| entry.id == payload.asset.id) {
        true => Vec::new(),
        false => vec![Op::from(crate::mutations::delete_asset::DeleteAsset { id: payload.asset.id.clone() })],
    })
}
