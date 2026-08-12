//! ↩️ `change-anchor-d-mm` inverse — restores the pre-change `anchor_d_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_anchor_d_mm::mutation::ChangeAnchorDMm;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorDMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorDMm(ChangeAnchorDMm { new_anchor_d_mm: base.anchor_d_mm.clone() })]
}
//#endregion 🔖️Inverse
