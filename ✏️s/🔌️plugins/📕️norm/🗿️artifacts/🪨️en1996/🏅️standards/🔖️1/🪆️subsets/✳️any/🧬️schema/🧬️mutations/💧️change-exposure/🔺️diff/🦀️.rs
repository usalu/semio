//! 🔺️ `change-exposure` sparse diff construction — writes only `En1996Diff.exposure` from the payload.

use crate::diff::En1996Diff;
use crate::mutations::change_exposure::ChangeExposure;
use crate::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeExposure, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.exposure == payload.new_exposure {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Exposure already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { exposure: Some(payload.new_exposure), ..Default::default() })
}
//#endregion 🔖️Diff
