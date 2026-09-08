//! ↩️ `change-hr-th` inverse — restores the pre-change `hr_t_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_hr_t_h::ChangeHrTH;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHrTH, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHrTH(ChangeHrTH { new_hr_t_h: base.hr_t_h })]
}
//#endregion 🔖️Inverse
