//! ↩️ `change-anchor-h-ef-mm` inverse — restores the pre-change `anchor_h_ef_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_anchor_h_ef_mm::ChangeAnchorHEfMm;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorHEfMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorHEfMm(ChangeAnchorHEfMm { new_anchor_h_ef_mm: base.anchor_h_ef_mm })]
}
//#endregion 🔖️Inverse
