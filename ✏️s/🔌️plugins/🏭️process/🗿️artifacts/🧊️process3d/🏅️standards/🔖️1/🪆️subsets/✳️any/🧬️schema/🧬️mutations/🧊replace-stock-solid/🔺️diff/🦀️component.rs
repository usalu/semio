//! 🔺️ `replace-stock-solid` sparse diff construction — a single `stock_solid` handle swap, never
//! a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::replace_stock_solid::mutation::ReplaceStockSolid;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceStockSolid, _base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { stock_solid: Some(payload.new_solid.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
