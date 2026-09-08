//! 🔺️ `change-multiple-resisting-systems` sparse diff construction — writes only `En1998Diff.multiple_resisting_systems` from the payload.

use crate::diff::En1998Diff;
use crate::mutations::change_multiple_resisting_systems::ChangeMultipleResistingSystems;
use crate::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMultipleResistingSystems, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.multiple_resisting_systems == payload.new_multiple_resisting_systems {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Multiple resisting systems flag is already {}.", payload.new_multiple_resisting_systems));
    }
    protocol::MutationOutcome::new(En1998Diff { multiple_resisting_systems: Some(payload.new_multiple_resisting_systems), ..Default::default() })
}
//#endregion 🔖️Diff
