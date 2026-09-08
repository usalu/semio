//! ↩️ `change-hr-delta-tc` inverse — restores the pre-change `hr_delta_t_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_hr_delta_t_c::ChangeHrDeltaTC;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHrDeltaTC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHrDeltaTC(ChangeHrDeltaTC { new_hr_delta_t_c: base.hr_delta_t_c })]
}
//#endregion 🔖️Inverse
