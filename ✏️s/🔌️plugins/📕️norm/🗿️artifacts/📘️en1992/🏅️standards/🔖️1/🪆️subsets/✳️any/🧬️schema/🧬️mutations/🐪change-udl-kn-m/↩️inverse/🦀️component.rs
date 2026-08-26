//! ↩️ `change-udl-kn-m` inverse — restores the pre-change `udl_kn_m` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_udl_kn_m::mutation::ChangeUdlKnM;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeUdlKnM, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeUdlKnM(ChangeUdlKnM { new_udl_kn_m: base.udl_kn_m.clone() })]
}
//#endregion 🔖️Inverse
