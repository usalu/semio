//! 🔺️ `change-z-mm3` sparse diff construction — writes only `En1996Diff.z_mm3` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_z_mm3::ChangeZMm3;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeZMm3, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_z_mm3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Z mm3 must be a finite number.", Vec::<String>::new());
    }
    if base.z_mm3 == payload.new_z_mm3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Z mm3 already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { z_mm3: Some(payload.new_z_mm3.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
