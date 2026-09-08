//! ↩ Inverse constructor for `DragAssets` — reconstructed from BASE state.

use super::DragAssets;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &DragAssets, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DragAssets(DragAssets { asset_ids: payload.asset_ids.clone(), dx: -payload.dx, dy: -payload.dy, dz: -payload.dz })]
}
