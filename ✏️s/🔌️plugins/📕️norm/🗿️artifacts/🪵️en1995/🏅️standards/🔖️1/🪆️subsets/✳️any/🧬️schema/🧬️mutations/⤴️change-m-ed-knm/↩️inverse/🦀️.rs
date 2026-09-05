//! ↩️ `change-m-ed-knm` inverse — restores the pre-change `m_ed_knm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_m_ed_knm::ChangeMEdKnm;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMEdKnm, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: base.m_ed_knm.clone() })]
}
//#endregion 🔖️Inverse
