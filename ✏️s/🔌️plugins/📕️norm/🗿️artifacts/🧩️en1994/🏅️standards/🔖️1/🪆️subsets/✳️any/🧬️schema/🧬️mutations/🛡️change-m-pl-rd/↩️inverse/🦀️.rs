//! ↩️ `change-m-pl-rd` — undo restores BASE's m_pl_rd.

use super::ChangeMPlRd;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMPlRd, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeMPlRd(ChangeMPlRd { new_m_pl_rd: base.m_pl_rd })]
}
//#endregion 🔖️Inverse
