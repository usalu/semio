//! ↩ Inverse constructor for `RotateAssets` — the negated angle around the same axis.

use super::mutation::RotateAssets;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 🔄️RotateAssets
pub fn inverse_rotate_assets(payload: &RotateAssets, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::RotateAssets(RotateAssets { asset_ids: payload.asset_ids.clone(), ax: payload.ax, ay: payload.ay, az: payload.az, angle: -payload.angle })]
}
//#endregion 🔄️RotateAssets
