//! ↩ Inverse constructor for `RotateAssets` — reconstructed from BASE state.

use super::RotateAssets;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &RotateAssets, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    let asset_ids: Vec<String> = base.assets.iter().filter(|asset| payload.asset_ids.contains(&asset.id)).map(|asset| asset.id.clone()).collect();
    Ok(match asset_ids.is_empty() {
        true => Vec::new(),
        false => vec![ShootingMutation::RotateAssets(RotateAssets { asset_ids, ax: payload.ax, ay: payload.ay, az: payload.az, angle: -payload.angle })],
    })
}
