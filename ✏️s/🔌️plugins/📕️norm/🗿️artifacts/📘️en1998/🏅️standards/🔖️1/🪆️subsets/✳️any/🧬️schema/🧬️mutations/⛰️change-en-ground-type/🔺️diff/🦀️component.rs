//! 🔺️ `change-en-ground-type` sparse diff construction — writes only `En1998Diff.en_ground_type` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_en_ground_type::mutation::ChangeEnGroundType;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnGroundType, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.en_ground_type == payload.new_en_ground_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("EN ground type is already \"{}\".", payload.new_en_ground_type));
    }
    protocol::MutationOutcome::new(En1998Diff { en_ground_type: Some(payload.new_en_ground_type.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
