//! ↩️ `change-es-mpa` inverse — restores the pre-change `e_s_mpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_e_s_mpa::ChangeESMpa;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeESMpa, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeESMpa(ChangeESMpa { new_e_s_mpa: base.e_s_mpa.clone() })]
}
//#endregion 🔖️Inverse
