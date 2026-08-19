//! 🔺️ `change-exposure` sparse diff construction — writes only `En1996Diff.exposure` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_exposure::mutation::ChangeExposure;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeExposure, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.exposure == payload.new_exposure {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Exposure already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { exposure: Some(payload.new_exposure.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
