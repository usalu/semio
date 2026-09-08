//! ↩️ `change-f-vk-mpa` inverse — restores the pre-change `f_vk_mpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_f_vk_mpa::ChangeFVkMpa;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFVkMpa, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeFVkMpa(ChangeFVkMpa { new_f_vk_mpa: base.f_vk_mpa })]
}
//#endregion 🔖️Inverse
