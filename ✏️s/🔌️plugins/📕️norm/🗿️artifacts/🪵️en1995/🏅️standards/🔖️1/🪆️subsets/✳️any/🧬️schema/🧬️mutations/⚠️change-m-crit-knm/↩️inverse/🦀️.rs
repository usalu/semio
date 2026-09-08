//! ↩️ `change-m-crit-knm` inverse — restores the pre-change `m_crit_knm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_m_crit_knm::ChangeMCritKnm;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMCritKnm, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeMCritKnm(ChangeMCritKnm { new_m_crit_knm: base.m_crit_knm })]
}
//#endregion 🔖️Inverse
