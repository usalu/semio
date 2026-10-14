//! 🔺️ `change-storeys` diff — sets the document's storey count; the same value is a `mutation.no-op`.

use crate::change_storeys::ChangeStoreys;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeStoreys, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.storeys == payload.new_storeys {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The storey count already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { storeys: Some(payload.new_storeys), ..Default::default() })
}
