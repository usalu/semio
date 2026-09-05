//! 🔺️ `change-section-depth-mm` sparse diff construction — writes only `En1995Diff.section_depth_mm` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_section_depth_mm::ChangeSectionDepthMm;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSectionDepthMm, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_section_depth_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Section depth mm must be a finite number.", Vec::<String>::new());
    }
    if base.section_depth_mm == payload.new_section_depth_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Section depth mm already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { section_depth_mm: Some(payload.new_section_depth_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
