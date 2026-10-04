//! ↩ Inverse constructor for `ScaleAssets` — reconstructed from BASE state.

use super::ScaleAssets;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &ScaleAssets, _base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    fn reciprocal(value: f64) -> f64 {
        if value.abs() < 1e-8 {
            1.0
        } else {
            1.0 / value
        }
    }
    vec![ShootingMutation::ScaleAssets(ScaleAssets { asset_ids: payload.asset_ids.clone(), sx: reciprocal(payload.sx), sy: reciprocal(payload.sy), sz: reciprocal(payload.sz) })]

    })())
}
