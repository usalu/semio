//! ↩️ `change-theta-c` inverse — restores the pre-change `theta_c` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_theta_c::mutation::ChangeThetaC;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeThetaC, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeThetaC(ChangeThetaC { new_theta_c: base.theta_c.clone() })]
}
//#endregion 🔖️Inverse
