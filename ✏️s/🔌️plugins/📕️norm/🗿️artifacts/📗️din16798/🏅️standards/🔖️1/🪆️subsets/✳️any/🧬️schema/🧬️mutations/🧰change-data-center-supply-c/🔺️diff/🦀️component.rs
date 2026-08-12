//! 🔺️ `change-data-center-supply-c` sparse diff construction — writes only `Din16798Diff.data_center_supply_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_data_center_supply_c::mutation::ChangeDataCenterSupplyC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDataCenterSupplyC, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { data_center_supply_c: Some(payload.new_data_center_supply_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
