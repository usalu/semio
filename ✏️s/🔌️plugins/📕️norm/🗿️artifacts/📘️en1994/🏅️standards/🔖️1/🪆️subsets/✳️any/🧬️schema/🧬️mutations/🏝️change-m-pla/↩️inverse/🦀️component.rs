//! ↩️ `change-m-pla` — undo restores BASE's m_pla.

use super::mutation::ChangeMPla;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeMPla, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeMPla(ChangeMPla { new_m_pla: base.m_pla })]
}
//#endregion 🔖️Inverse
