//! 🔺️ `change-load-duration` sparse diff construction — writes only `En1995Diff.load_duration` from the payload.

use crate::diff::En1995Diff;
use crate::mutations::change_load_duration::ChangeLoadDuration;
use crate::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadDuration, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.load_duration == payload.new_load_duration {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Load duration already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { load_duration: Some(payload.new_load_duration.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
