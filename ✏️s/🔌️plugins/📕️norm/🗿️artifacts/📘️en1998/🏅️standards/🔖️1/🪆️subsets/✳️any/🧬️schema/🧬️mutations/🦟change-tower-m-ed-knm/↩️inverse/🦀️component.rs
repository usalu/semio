//! ↩️ `change-tower-m-ed-knm` inverse — restores the pre-change `tower_m_ed_knm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_tower_m_ed_knm::mutation::ChangeTowerMEdKnm;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeTowerMEdKnm, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTowerMEdKnm(ChangeTowerMEdKnm { new_tower_m_ed_knm: base.tower_m_ed_knm.clone() })]
}
//#endregion 🔖️Inverse
