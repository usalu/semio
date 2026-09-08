//! ↩️ `change-data-center-supply-c` inverse — restores the pre-change `data_center_supply_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_data_center_supply_c::ChangeDataCenterSupplyC;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDataCenterSupplyC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeDataCenterSupplyC(ChangeDataCenterSupplyC { new_data_center_supply_c: base.data_center_supply_c })]
}
//#endregion 🔖️Inverse
