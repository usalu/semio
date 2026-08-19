//! ↩️ `change-anchor-as-mm2` inverse — restores the pre-change `anchor_a_s_mm2` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_anchor_a_s_mm2::mutation::ChangeAnchorASMm2;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAnchorASMm2, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorASMm2(ChangeAnchorASMm2 { new_anchor_a_s_mm2: base.anchor_a_s_mm2.clone() })]
}
//#endregion 🔖️Inverse
