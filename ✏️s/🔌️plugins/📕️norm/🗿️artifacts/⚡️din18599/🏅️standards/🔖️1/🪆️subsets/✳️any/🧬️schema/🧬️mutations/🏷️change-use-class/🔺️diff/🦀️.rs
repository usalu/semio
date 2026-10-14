//! 🔺️ `change-use-class` sparse diff.

use semio_s_artifact_norm_din18599_diff::Din18599Diff;
use crate::change_use_class::ChangeUseClass;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn diff(payload: &ChangeUseClass, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.use_class == payload.new_use_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "use-class already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { use_class: Some(payload.new_use_class), ..Default::default() })
}
