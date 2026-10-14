//! 🔺️ `change-method` sparse diff.

use semio_s_artifact_norm_din18599_diff::Din18599Diff;
use crate::change_method::ChangeMethod;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn diff(payload: &ChangeMethod, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.method == payload.new_method {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "method already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { method: Some(payload.new_method), ..Default::default() })
}
