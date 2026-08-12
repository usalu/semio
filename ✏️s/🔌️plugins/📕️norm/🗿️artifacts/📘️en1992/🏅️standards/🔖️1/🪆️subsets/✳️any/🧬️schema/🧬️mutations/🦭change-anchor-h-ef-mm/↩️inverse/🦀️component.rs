//! ↩️ `change-anchor-h-ef-mm` inverse — restores the pre-change `anchor_h_ef_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_anchor_h_ef_mm::mutation::ChangeAnchorHEfMm;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorHEfMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorHEfMm(ChangeAnchorHEfMm { new_anchor_h_ef_mm: base.anchor_h_ef_mm.clone() })]
}
//#endregion 🔖️Inverse
