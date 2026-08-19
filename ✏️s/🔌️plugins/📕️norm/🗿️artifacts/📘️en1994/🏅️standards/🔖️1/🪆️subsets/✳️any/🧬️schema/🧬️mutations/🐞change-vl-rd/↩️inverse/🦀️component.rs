//! ↩️ `change-vl-rd` — undo restores BASE's v_l_rd.

use super::mutation::ChangeVLRd;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeVLRd, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeVLRd(ChangeVLRd { new_v_l_rd: base.v_l_rd })]
}
//#endregion 🔖️Inverse
