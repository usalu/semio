//! ↩️ `change-anchor-c1-mm` inverse — restores the pre-change `anchor_c1_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_anchor_c1_mm::ChangeAnchorC1Mm;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorC1Mm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorC1Mm(ChangeAnchorC1Mm { new_anchor_c1_mm: base.anchor_c1_mm })]
}
//#endregion 🔖️Inverse
