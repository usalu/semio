//! ↩ Inverse constructor for `RotateAssets` — reconstructed from BASE state.

use super::RotateAssets;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &RotateAssets, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::RotateAssets(RotateAssets { asset_ids: payload.asset_ids.clone(), ax: payload.ax, ay: payload.ay, az: payload.az, angle: -payload.angle })]
}
