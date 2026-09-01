//! 🔺️ `change-unit` sparse diff construction — writes only `En1996Diff.unit` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_unit::ChangeUnit;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeUnit, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.unit == payload.new_unit {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Unit already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { unit: Some(payload.new_unit.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
