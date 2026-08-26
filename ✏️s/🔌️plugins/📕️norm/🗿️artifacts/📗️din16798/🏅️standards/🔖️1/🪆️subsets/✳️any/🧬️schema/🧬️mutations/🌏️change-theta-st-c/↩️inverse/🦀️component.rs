//! ↩️ `change-theta-st-c` inverse — restores the pre-change `theta_st_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_theta_st_c::mutation::ChangeThetaStC;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeThetaStC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeThetaStC(ChangeThetaStC { new_theta_st_c: base.theta_st_c.clone() })]
}
//#endregion 🔖️Inverse
