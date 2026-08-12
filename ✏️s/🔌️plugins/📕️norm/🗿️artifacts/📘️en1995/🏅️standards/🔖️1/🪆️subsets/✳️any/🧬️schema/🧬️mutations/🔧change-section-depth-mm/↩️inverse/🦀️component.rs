//! ↩️ `change-section-depth-mm` inverse — restores the pre-change `section_depth_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_section_depth_mm::mutation::ChangeSectionDepthMm;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSectionDepthMm, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeSectionDepthMm(ChangeSectionDepthMm { new_section_depth_mm: base.section_depth_mm.clone() })]
}
//#endregion 🔖️Inverse
