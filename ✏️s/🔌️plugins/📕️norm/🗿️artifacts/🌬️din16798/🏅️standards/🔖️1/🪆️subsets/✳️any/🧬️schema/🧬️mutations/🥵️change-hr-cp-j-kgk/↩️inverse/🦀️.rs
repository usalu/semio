//! ↩️ `change-hr-cp-j-kgk` inverse — restores the pre-change `hr_cp_j_kgk` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_hr_cp_j_kgk::ChangeHrCpJKgk;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHrCpJKgk, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHrCpJKgk(ChangeHrCpJKgk { new_hr_cp_j_kgk: base.hr_cp_j_kgk })]
}
//#endregion 🔖️Inverse
