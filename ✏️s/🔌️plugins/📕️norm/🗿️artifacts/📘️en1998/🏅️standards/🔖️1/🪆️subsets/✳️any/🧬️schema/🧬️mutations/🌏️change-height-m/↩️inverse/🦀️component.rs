//! ↩️ `change-height-m` inverse — restores the pre-change `height_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_height_m::mutation::ChangeHeightM;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHeightM, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeHeightM(ChangeHeightM { new_height_m: base.height_m.clone() })]
}
//#endregion 🔖️Inverse
