//! ↩ Inverse constructor for `DeleteAsset` — reconstructed from BASE state.

use super::DeleteAsset;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &DeleteAsset, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset>,
{
    Ok(match base.assets.iter().position(|entry| entry.id == payload.id) {
        Some(index) => vec![Op::from(crate::mutations::create_asset::CreateAsset { asset: base.assets[index].clone(), index: Some(index) })],
        None => Vec::new(),
    })
}
