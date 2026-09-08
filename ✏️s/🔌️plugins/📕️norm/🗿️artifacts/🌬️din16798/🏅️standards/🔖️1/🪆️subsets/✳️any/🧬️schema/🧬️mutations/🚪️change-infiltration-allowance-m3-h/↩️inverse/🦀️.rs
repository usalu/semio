//! ↩️ `change-infiltration-allowance-m3-h` inverse — restores the pre-change `infiltration_allowance_m3_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_infiltration_allowance_m3_h::ChangeInfiltrationAllowanceM3H;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeInfiltrationAllowanceM3H, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeInfiltrationAllowanceM3H(ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h: base.infiltration_allowance_m3_h })]
}
//#endregion 🔖️Inverse
