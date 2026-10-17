//! ↩ Inverse constructor for `DragAssets` — reconstructed from BASE state.

use super::DragAssets;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &DragAssets, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::drag_assets::DragAssets>,
{
    let asset_ids: Vec<String> = base.assets.iter().filter(|asset| payload.asset_ids.contains(&asset.id)).map(|asset| asset.id.clone()).collect();
    Ok(match asset_ids.is_empty() {
        true => Vec::new(),
        false => vec![Op::from(DragAssets { asset_ids, dx: -payload.dx, dy: -payload.dy, dz: -payload.dz })],
    })
}
