//! 🔺️ `replace-stock-solid` sparse diff construction — a whole-`Stock` value with only `solid`
//! replaced from `base`, never a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::replace_stock_solid::mutation::ReplaceStockSolid;
use crate::artifacts::process3d::{Process3dSnapshot, Stock};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceStockSolid, base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { stock: Some(Stock { solid: payload.new_solid.clone(), ..base.stock.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
