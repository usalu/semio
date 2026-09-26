//! 🔺️ `change-use-class` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::change_use_class::ChangeUseClass;
use crate::Din18599Snapshot;

pub fn diff(payload: &ChangeUseClass, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.use_class == payload.new_use_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "use-class already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { use_class: Some(payload.new_use_class), ..Default::default() })
}
