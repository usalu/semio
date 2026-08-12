//! 🔺 Diff constructor for `SetActiveAsset`. `ShootingSnapshot::active_asset_id` is a required
//! (non-`Option`) `String` field, empty meaning "none" — `None` encodes to the empty string.

use super::mutation::SetActiveAsset;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 📌️SetActiveAsset
pub fn diff_set_active_asset(payload: &SetActiveAsset, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { active_asset_id: Some(payload.asset_id.clone().unwrap_or_default()), ..Default::default() }
}
//#endregion 📌️SetActiveAsset
