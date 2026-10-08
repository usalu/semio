//! ↩ Inverse constructor for `DragAssets` — reconstructed from BASE state.

use super::DragAssets;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &DragAssets, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    let asset_ids: Vec<String> = base.assets.iter().filter(|asset| payload.asset_ids.contains(&asset.id)).map(|asset| asset.id.clone()).collect();
    Ok(match asset_ids.is_empty() {
        true => Vec::new(),
        false => vec![ShootingMutation::DragAssets(DragAssets { asset_ids, dx: -payload.dx, dy: -payload.dy, dz: -payload.dz })],
    })
}
