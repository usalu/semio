//! ↩️ `change-a-vert-m-s2` inverse — restores the pre-change `a_vert_m_s2` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_a_vert_m_s2::mutation::ChangeAVertMS2;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAVertMS2, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeAVertMS2(ChangeAVertMS2 { new_a_vert_m_s2: base.a_vert_m_s2.clone() })]
}
//#endregion 🔖️Inverse
