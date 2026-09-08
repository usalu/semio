//! ↩️ `change-theta-amb-c` inverse — restores the pre-change `theta_amb_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_theta_amb_c::ChangeThetaAmbC;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeThetaAmbC, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeThetaAmbC(ChangeThetaAmbC { new_theta_amb_c: base.theta_amb_c })]
}
//#endregion 🔖️Inverse
