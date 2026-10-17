//! ↩ Inverse constructor for `ReorderAssets` — reconstructed from BASE state.

use super::ReorderAssets;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ReorderAssets, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::reorder_assets::ReorderAssets>,
{
    Ok((|| {
    match base.assets.iter().position(|asset| asset.id == payload.id) {
        Some(original_index) => vec![Op::from(ReorderAssets { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }

    })())
}
