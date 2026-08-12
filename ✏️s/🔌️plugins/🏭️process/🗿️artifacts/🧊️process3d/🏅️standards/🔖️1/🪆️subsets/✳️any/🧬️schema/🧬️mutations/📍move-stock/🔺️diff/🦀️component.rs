//! 🔺️ `move-stock` sparse diff construction — a whole-`Stock` value with only `pose` replaced from
//! `base`, never a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::move_stock::mutation::MoveStock;
use crate::artifacts::process3d::{Process3dSnapshot, Stock};

//#region 🔖️Diff
pub fn diff(payload: &MoveStock, base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { stock: Some(Stock { pose: payload.new_pose.clone(), ..base.stock.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
