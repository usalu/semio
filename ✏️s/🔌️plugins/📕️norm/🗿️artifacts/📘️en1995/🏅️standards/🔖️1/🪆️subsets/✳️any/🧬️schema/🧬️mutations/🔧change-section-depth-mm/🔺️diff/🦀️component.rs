//! 🔺️ `change-section-depth-mm` sparse diff construction — writes only `En1995Diff.section_depth_mm` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_section_depth_mm::mutation::ChangeSectionDepthMm;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSectionDepthMm, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { section_depth_mm: Some(payload.new_section_depth_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
