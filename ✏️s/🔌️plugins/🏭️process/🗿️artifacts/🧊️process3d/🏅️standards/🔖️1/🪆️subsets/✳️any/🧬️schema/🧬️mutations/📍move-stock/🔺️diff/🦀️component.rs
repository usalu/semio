//! 🔺️ `move-stock` sparse diff construction — a whole-`Stock` value with only `pose` replaced from
//! `base`, never a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::move_stock::mutation::MoveStock;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveStock, _base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { stock_pose: Some(payload.new_pose.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
