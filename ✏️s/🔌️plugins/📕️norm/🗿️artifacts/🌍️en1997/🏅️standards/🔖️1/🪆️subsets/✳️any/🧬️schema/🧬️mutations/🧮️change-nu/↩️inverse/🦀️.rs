//! ↩️ `change-nu` inverse — restores the pre-change `nu` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_nu::ChangeNu;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeNu, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeNu(ChangeNu { new_nu: base.nu })]
}
//#endregion 🔖️Inverse
