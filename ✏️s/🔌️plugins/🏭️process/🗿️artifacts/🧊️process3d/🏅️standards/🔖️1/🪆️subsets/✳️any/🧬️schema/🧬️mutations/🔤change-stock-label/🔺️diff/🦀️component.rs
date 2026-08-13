//! 🔺️ `change-stock-label` sparse diff construction — a whole-`Stock` value with only `label`
//! replaced from `base`, never a snapshot clone.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::change_stock_label::mutation::ChangeStockLabel;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStockLabel, _base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { stock_label: Some(payload.new_label.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
