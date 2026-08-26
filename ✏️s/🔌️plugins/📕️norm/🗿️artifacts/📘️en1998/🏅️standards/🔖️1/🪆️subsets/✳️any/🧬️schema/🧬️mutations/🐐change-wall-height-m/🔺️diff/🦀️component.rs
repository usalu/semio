//! 🔺️ `change-wall-height-m` sparse diff construction — writes only `En1998Diff.wall_height_m` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_wall_height_m::mutation::ChangeWallHeightM;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWallHeightM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_wall_height_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Retaining wall height [m] must be a finite number, got {}.", payload.new_wall_height_m), Vec::<String>::new());
    }
    if base.wall_height_m == payload.new_wall_height_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Retaining wall height [m] is already {}.", payload.new_wall_height_m));
    }
    protocol::MutationOutcome::new(En1998Diff { wall_height_m: Some(payload.new_wall_height_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
