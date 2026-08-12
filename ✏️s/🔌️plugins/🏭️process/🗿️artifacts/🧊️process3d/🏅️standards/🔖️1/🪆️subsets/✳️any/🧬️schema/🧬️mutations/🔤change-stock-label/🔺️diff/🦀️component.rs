//! 🔺️ `change-stock-label` sparse diff construction — a whole-`Stock` value with only `label`
//! replaced from `base`, never a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::change_stock_label::mutation::ChangeStockLabel;
use crate::artifacts::process3d::{Process3dSnapshot, Stock};

//#region 🔖️Diff
pub fn diff(payload: &ChangeStockLabel, base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { stock: Some(Stock { label: payload.new_label.clone(), ..base.stock.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
