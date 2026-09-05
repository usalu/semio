//! ↩️ `change-m-crit-knm` inverse — restores the pre-change `m_crit_knm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_m_crit_knm::ChangeMCritKnm;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMCritKnm, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeMCritKnm(ChangeMCritKnm { new_m_crit_knm: base.m_crit_knm.clone() })]
}
//#endregion 🔖️Inverse
