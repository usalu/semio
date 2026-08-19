//! ↩️ `change-nu` inverse — restores the pre-change `nu` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_nu::mutation::ChangeNu;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeNu, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeNu(ChangeNu { new_nu: base.nu.clone() })]
}
//#endregion 🔖️Inverse
