//! ↩ Inverse constructor for `DragAssets` — reconstructed from BASE state.

use super::mutation::DragAssets;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub async fn inverse(payload: &DragAssets, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DragAssets(DragAssets { asset_ids: payload.asset_ids.clone(), dx: -payload.dx, dy: -payload.dy, dz: -payload.dz })]
}
