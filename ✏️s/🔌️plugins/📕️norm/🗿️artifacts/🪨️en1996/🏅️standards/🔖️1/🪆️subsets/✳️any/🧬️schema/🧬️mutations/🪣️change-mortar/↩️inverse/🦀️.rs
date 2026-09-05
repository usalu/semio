//! ↩️ `change-mortar` inverse — restores the pre-change `mortar` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_mortar::ChangeMortar;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMortar, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeMortar(ChangeMortar { new_mortar: base.mortar.clone() })]
}
//#endregion 🔖️Inverse
