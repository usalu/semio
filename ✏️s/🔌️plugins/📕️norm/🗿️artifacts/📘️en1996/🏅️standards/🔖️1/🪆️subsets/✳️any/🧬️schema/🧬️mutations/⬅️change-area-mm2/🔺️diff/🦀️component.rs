//! 🔺️ `change-area-mm2` sparse diff construction — writes only `En1996Diff.area_mm2` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_area_mm2::mutation::ChangeAreaMm2;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAreaMm2, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_area_mm2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Area mm2 must be a finite number.", Vec::<String>::new());
    }
    if base.area_mm2 == payload.new_area_mm2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Area mm2 already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { area_mm2: Some(payload.new_area_mm2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
