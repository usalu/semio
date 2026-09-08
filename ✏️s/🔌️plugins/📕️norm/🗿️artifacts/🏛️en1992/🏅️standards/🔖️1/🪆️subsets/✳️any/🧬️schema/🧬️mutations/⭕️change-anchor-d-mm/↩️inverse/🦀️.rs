//! ↩️ `change-anchor-d-mm` inverse — restores the pre-change `anchor_d_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_anchor_d_mm::ChangeAnchorDMm;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorDMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorDMm(ChangeAnchorDMm { new_anchor_d_mm: base.anchor_d_mm })]
}
//#endregion 🔖️Inverse
