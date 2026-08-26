//! ↩️ `change-m-ed-knm` inverse — restores the pre-change `m_ed_knm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_m_ed_knm::mutation::ChangeMEdKnm;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMEdKnm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: base.m_ed_knm.clone() })]
}
//#endregion 🔖️Inverse
