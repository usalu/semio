//! ↩️ `change-z-mm3` inverse — restores the pre-change `z_mm3` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_z_mm3::mutation::ChangeZMm3;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeZMm3, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeZMm3(ChangeZMm3 { new_z_mm3: base.z_mm3.clone() })]
}
//#endregion 🔖️Inverse
