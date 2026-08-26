//! 🔺️ `change-alloy` sparse diff construction — writes only `En1999Diff.alloy` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_alloy::mutation::ChangeAlloy;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAlloy, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.alloy == payload.new_alloy {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Aluminium alloy designation is already \"{}\".", payload.new_alloy));
    }
    protocol::MutationOutcome::new(En1999Diff { alloy: Some(payload.new_alloy.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
