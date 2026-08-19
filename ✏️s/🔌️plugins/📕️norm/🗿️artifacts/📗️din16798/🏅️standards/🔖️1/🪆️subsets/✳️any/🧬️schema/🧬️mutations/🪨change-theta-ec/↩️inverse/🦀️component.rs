//! ↩️ `change-theta-ec` inverse — restores the pre-change `theta_e_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_theta_e_c::mutation::ChangeThetaEC;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeThetaEC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeThetaEC(ChangeThetaEC { new_theta_e_c: base.theta_e_c.clone() })]
}
//#endregion 🔖️Inverse
