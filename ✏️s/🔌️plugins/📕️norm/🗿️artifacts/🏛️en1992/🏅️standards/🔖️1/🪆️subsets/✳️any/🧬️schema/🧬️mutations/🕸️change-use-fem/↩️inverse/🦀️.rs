//! ↩️ `change-use-fem` inverse — restores the pre-change `use_fem` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_use_fem::ChangeUseFem;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeUseFem, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeUseFem(ChangeUseFem { new_use_fem: base.use_fem })]
}
//#endregion 🔖️Inverse
