//! 🔺️ `change-persons` sparse diff construction — writes only `Din16798Diff.persons` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_persons::mutation::ChangePersons;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangePersons, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.persons == payload.new_persons {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Number of persons is already {}.", payload.new_persons));
    }
    protocol::MutationOutcome::new(Din16798Diff { persons: Some(payload.new_persons.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
