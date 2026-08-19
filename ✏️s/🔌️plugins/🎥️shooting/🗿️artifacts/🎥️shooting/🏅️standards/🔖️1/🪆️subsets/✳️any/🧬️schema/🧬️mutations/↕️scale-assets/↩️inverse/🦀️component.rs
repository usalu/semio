//! ↩ Inverse constructor for `ScaleAssets` — reconstructed from BASE state.

use super::mutation::ScaleAssets;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub async fn inverse(payload: &ScaleAssets, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    async fn reciprocal(value: f64) -> f64 {
        if value.abs() < 1e-8 { 1.0 } else { 1.0 / value }
    }
    vec![ShootingMutation::ScaleAssets(ScaleAssets { asset_ids: payload.asset_ids.clone(), sx: reciprocal(payload.sx), sy: reciprocal(payload.sy), sz: reciprocal(payload.sz) })]
}
