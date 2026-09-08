//! ↩️ `change-theta-rm-c` inverse — restores the pre-change `theta_rm_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_theta_rm_c::ChangeThetaRmC;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeThetaRmC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeThetaRmC(ChangeThetaRmC { new_theta_rm_c: base.theta_rm_c })]
}
//#endregion 🔖️Inverse
