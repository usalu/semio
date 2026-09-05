//! 🔺️ `change-ground-type` sparse diff construction — writes only `En1998Diff.ground_type` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_ground_type::ChangeGroundType;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGroundType, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.ground_type == payload.new_ground_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Ground type is already \"{}\".", payload.new_ground_type));
    }
    protocol::MutationOutcome::new(En1998Diff { ground_type: Some(payload.new_ground_type.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
