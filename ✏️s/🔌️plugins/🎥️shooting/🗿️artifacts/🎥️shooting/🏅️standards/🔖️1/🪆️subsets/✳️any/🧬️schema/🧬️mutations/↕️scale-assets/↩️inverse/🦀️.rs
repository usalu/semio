//! ↩ Inverse constructor for `ScaleAssets` — reconstructed from BASE state.

use super::ScaleAssets;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

fn reciprocal(value: f64) -> f64 {
    if value.abs() < 1e-8 {
        1.0
    } else {
        1.0 / value
    }
}

pub fn inverse(payload: &ScaleAssets, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    let asset_ids: Vec<String> = base.assets.iter().filter(|asset| payload.asset_ids.contains(&asset.id)).map(|asset| asset.id.clone()).collect();
    Ok(match asset_ids.is_empty() {
        true => Vec::new(),
        false => vec![ShootingMutation::ScaleAssets(ScaleAssets { asset_ids, sx: reciprocal(payload.sx), sy: reciprocal(payload.sy), sz: reciprocal(payload.sz) })],
    })
}
