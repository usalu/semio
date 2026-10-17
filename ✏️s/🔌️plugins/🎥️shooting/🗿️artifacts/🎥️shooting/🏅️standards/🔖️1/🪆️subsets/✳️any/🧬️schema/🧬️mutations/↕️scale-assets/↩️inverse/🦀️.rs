//! ↩ Inverse constructor for `ScaleAssets` — reconstructed from BASE state.

use super::ScaleAssets;
use crate::ShootingSnapshot;

fn reciprocal(value: f64) -> f64 {
    if value.abs() < 1e-8 {
        1.0
    } else {
        1.0 / value
    }
}

pub fn inverse<Op>(payload: &ScaleAssets, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::scale_assets::ScaleAssets>,
{
    let asset_ids: Vec<String> = base.assets.iter().filter(|asset| payload.asset_ids.contains(&asset.id)).map(|asset| asset.id.clone()).collect();
    Ok(match asset_ids.is_empty() {
        true => Vec::new(),
        false => vec![Op::from(ScaleAssets { asset_ids, sx: reciprocal(payload.sx), sy: reciprocal(payload.sy), sz: reciprocal(payload.sz) })],
    })
}
