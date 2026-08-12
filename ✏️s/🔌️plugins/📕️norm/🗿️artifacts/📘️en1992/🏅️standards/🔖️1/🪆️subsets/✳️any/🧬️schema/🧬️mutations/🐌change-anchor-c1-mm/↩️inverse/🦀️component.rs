//! ↩️ `change-anchor-c1-mm` inverse — restores the pre-change `anchor_c1_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_anchor_c1_mm::mutation::ChangeAnchorC1Mm;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorC1Mm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorC1Mm(ChangeAnchorC1Mm { new_anchor_c1_mm: base.anchor_c1_mm.clone() })]
}
//#endregion 🔖️Inverse
