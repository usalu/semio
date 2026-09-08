//! 🔺️ `change-use-fem` sparse diff construction — writes only `En1992Diff.use_fem` from the payload.

use crate::diff::En1992Diff;
use crate::mutations::change_use_fem::ChangeUseFem;
use crate::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeUseFem, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.use_fem == payload.new_use_fem {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Use fem already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { use_fem: Some(payload.new_use_fem), ..Default::default() })
}
//#endregion 🔖️Diff
