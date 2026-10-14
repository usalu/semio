//! 🔺️ `change-usage` diff.

use crate::change_usage::ChangeUsage;
use semio_s_artifact_norm_din4108_diff::Din4108Diff;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn diff(payload: &ChangeUsage, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.usage == payload.new_usage {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "usage already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { usage: Some(payload.new_usage.clone()), ..Default::default() })
}
