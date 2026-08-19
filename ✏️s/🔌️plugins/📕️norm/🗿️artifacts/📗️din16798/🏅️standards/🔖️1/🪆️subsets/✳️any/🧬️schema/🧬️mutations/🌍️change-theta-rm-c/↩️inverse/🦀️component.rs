//! ↩️ `change-theta-rm-c` inverse — restores the pre-change `theta_rm_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_theta_rm_c::mutation::ChangeThetaRmC;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeThetaRmC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeThetaRmC(ChangeThetaRmC { new_theta_rm_c: base.theta_rm_c.clone() })]
}
//#endregion 🔖️Inverse
