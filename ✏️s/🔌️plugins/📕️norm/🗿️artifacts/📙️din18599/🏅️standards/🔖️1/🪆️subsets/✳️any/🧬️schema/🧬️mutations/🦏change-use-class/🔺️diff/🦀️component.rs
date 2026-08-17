//! 🔺️ `change-use-class` sparse diff construction — writes only `Din18599Diff.use_class` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_use_class::mutation::ChangeUseClass;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeUseClass, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if base.use_class == payload.new_use_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Use class already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { use_class: Some(payload.new_use_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
