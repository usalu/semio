//! ↩️ `change-theta-amb-c` inverse — restores the pre-change `theta_amb_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_theta_amb_c::mutation::ChangeThetaAmbC;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeThetaAmbC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeThetaAmbC(ChangeThetaAmbC { new_theta_amb_c: base.theta_amb_c.clone() })]
}
//#endregion 🔖️Inverse
