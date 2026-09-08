//! ↩️ `change-theta-set-c` inverse — restores the pre-change `theta_set_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_theta_set_c::ChangeThetaSetC;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeThetaSetC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeThetaSetC(ChangeThetaSetC { new_theta_set_c: base.theta_set_c })]
}
//#endregion 🔖️Inverse
