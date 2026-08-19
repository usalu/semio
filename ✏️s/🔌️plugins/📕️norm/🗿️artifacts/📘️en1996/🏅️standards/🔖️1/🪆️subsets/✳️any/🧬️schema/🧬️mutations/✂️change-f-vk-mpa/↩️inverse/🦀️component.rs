//! ↩️ `change-f-vk-mpa` inverse — restores the pre-change `f_vk_mpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_f_vk_mpa::mutation::ChangeFVkMpa;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFVkMpa, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeFVkMpa(ChangeFVkMpa { new_f_vk_mpa: base.f_vk_mpa.clone() })]
}
//#endregion 🔖️Inverse
