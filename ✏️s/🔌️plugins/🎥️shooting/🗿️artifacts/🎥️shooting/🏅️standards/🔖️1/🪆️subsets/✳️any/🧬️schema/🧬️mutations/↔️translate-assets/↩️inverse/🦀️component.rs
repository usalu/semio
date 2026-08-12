//! ↩ Inverse constructor for `DragAssets` — the negated offset, always applicable (a relative
//! bulk gesture has no missing-target case: dragging an absent id is already a no-op in `diff`).

use super::mutation::DragAssets;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region ↔️DragAssets
pub fn inverse_drag_assets(payload: &DragAssets, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DragAssets(DragAssets { asset_ids: payload.asset_ids.clone(), dx: -payload.dx, dy: -payload.dy, dz: -payload.dz })]
}
//#endregion ↔️DragAssets
