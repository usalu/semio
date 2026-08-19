//! 🔺️ `change-storeys` sparse diff construction — writes only `En1996Diff.storeys` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_storeys::mutation::ChangeStoreys;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeStoreys, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.storeys == payload.new_storeys {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Storeys already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { storeys: Some(payload.new_storeys.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
