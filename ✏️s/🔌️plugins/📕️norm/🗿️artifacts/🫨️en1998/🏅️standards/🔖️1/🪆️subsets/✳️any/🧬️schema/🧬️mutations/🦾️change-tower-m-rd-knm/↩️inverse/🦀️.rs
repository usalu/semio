//! ↩️ `change-tower-m-rd-knm` inverse — restores the pre-change `tower_m_rd_knm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_tower_m_rd_knm::ChangeTowerMRdKnm;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTowerMRdKnm, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTowerMRdKnm(ChangeTowerMRdKnm { new_tower_m_rd_knm: base.tower_m_rd_knm })]
}
//#endregion 🔖️Inverse
