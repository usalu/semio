//! ↩️ `change-m-ed-knm` — undo restores BASE's m_ed_knm.

use super::ChangeMEdKnm;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMEdKnm, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: base.m_ed_knm })]
}
//#endregion 🔖️Inverse
