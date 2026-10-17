//! ↩ Inverse constructor for `RotateAssets` — reconstructed from BASE state.

use super::RotateAssets;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &RotateAssets, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rotate_assets::RotateAssets>,
{
    let asset_ids: Vec<String> = base.assets.iter().filter(|asset| payload.asset_ids.contains(&asset.id)).map(|asset| asset.id.clone()).collect();
    Ok(match asset_ids.is_empty() {
        true => Vec::new(),
        false => vec![Op::from(RotateAssets { asset_ids, ax: payload.ax, ay: payload.ay, az: payload.az, angle: -payload.angle })],
    })
}
