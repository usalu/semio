//! 🔺️ `change-persons` sparse diff construction — writes only `Din16798Diff.persons` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_persons::ChangePersons;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePersons, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.persons == payload.new_persons {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Number of persons is already {}.", payload.new_persons));
    }
    protocol::MutationOutcome::new(Din16798Diff { persons: Some(payload.new_persons), ..Default::default() })
}
//#endregion 🔖️Diff
