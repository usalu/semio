//! ↩️ `change-m-ed-knm` inverse — restores the pre-change `m_ed_knm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_m_ed_knm::mutation::ChangeMEdKnm;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMEdKnm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: base.m_ed_knm.clone() })]
}
//#endregion 🔖️Inverse
