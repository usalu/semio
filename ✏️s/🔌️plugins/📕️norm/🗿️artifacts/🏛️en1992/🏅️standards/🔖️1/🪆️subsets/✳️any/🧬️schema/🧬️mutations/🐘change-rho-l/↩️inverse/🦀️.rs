//! ↩️ `change-rho-l` inverse — restores the pre-change `rho_l` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_rho_l::ChangeRhoL;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeRhoL, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeRhoL(ChangeRhoL { new_rho_l: base.rho_l.clone() })]
}
//#endregion 🔖️Inverse
