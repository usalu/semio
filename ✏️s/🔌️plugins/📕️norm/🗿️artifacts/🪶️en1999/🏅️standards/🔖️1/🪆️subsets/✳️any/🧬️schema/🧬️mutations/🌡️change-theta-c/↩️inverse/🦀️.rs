//! ↩️ `change-theta-c` inverse — restores the pre-change `theta_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_theta_c::ChangeThetaC;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeThetaC, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeThetaC(ChangeThetaC { new_theta_c: base.theta_c })]
}
//#endregion 🔖️Inverse
