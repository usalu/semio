//! 🔺️ `change-wall-thickness-mm` sparse diff construction — writes only `En1996Diff.wall_thickness_mm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_wall_thickness_mm::ChangeWallThicknessMm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWallThicknessMm, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_wall_thickness_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Wall thickness mm must be a finite number.", Vec::<String>::new());
    }
    if base.wall_thickness_mm == payload.new_wall_thickness_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Wall thickness mm already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { wall_thickness_mm: Some(payload.new_wall_thickness_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
