//! ↩ Inverse constructor for `SetActiveAsset` — always applicable.

use super::mutation::SetActiveAsset;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 📌️SetActiveAsset
pub fn inverse_set_active_asset(_payload: &SetActiveAsset, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    let asset_id = if base.active_asset_id.is_empty() { None } else { Some(base.active_asset_id.clone()) };
    vec![ShootingMutation::SetActiveAsset(SetActiveAsset { asset_id })]
}
//#endregion 📌️SetActiveAsset
