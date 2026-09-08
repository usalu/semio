//! ↩️ `change-theta-ec` inverse — restores the pre-change `theta_e_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_theta_e_c::ChangeThetaEC;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeThetaEC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeThetaEC(ChangeThetaEC { new_theta_e_c: base.theta_e_c })]
}
//#endregion 🔖️Inverse
