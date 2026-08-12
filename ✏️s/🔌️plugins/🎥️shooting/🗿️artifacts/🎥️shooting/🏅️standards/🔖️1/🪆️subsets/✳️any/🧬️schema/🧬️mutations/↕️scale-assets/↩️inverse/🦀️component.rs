//! ↩ Inverse constructor for `ScaleAssets` — the reciprocal factor per axis (treating a
//! near-zero factor as identity, matching the pre-migration behavior — scaling to exactly zero has
//! no meaningful undo factor).

use super::mutation::ScaleAssets;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region ↕️ScaleAssets
fn reciprocal(value: f64) -> f64 {
    if value.abs() < 1e-8 {
        1.0
    } else {
        1.0 / value
    }
}

pub fn inverse_scale_assets(payload: &ScaleAssets, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ScaleAssets(ScaleAssets { asset_ids: payload.asset_ids.clone(), sx: reciprocal(payload.sx), sy: reciprocal(payload.sy), sz: reciprocal(payload.sz) })]
}
//#endregion ↕️ScaleAssets
