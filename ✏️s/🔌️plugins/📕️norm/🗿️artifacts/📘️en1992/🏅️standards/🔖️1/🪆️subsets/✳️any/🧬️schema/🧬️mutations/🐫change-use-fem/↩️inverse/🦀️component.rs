//! ↩️ `change-use-fem` inverse — restores the pre-change `use_fem` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_use_fem::mutation::ChangeUseFem;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeUseFem, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeUseFem(ChangeUseFem { new_use_fem: base.use_fem.clone() })]
}
//#endregion 🔖️Inverse
