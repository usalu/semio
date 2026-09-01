//! ↩️ `change-m-ed-knm` inverse — restores the pre-change `m_ed_knm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_m_ed_knm::ChangeMEdKnm;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMEdKnm, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: base.m_ed_knm.clone() })]
}
//#endregion 🔖️Inverse
