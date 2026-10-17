//! ↩ Inverse constructor for `RenameAsset` — reconstructed from BASE state.

use super::RenameAsset;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &RenameAsset, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_asset::RenameAsset>,
{
    Ok((|| {
    match base.assets.iter().find(|asset| asset.id == payload.id) {
        Some(asset) => vec![Op::from(RenameAsset { id: payload.id.clone(), new_name: asset.name.clone() })],
        None => Vec::new(),
    }

    })())
}
