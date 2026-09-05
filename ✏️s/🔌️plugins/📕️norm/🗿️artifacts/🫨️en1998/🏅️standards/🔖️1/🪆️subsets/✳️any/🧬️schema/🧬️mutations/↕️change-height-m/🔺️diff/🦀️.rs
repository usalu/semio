//! 🔺️ `change-height-m` sparse diff construction — writes only `En1998Diff.height_m` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_height_m::ChangeHeightM;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHeightM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_height_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Building height [m] must be a finite number, got {}.", payload.new_height_m), Vec::<String>::new());
    }
    if base.height_m == payload.new_height_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Building height [m] is already {}.", payload.new_height_m));
    }
    protocol::MutationOutcome::new(En1998Diff { height_m: Some(payload.new_height_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
