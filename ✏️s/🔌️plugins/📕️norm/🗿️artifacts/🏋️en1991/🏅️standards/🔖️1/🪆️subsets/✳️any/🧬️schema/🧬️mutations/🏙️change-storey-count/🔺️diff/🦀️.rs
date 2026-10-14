//! 🔺️ Diff for `change-storey-count`.
use crate::change_storey_count::ChangeStoreyCount;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeStoreyCount, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.storey_count == payload.new_storey_count {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { storey_count: Some(payload.new_storey_count), ..Default::default() })
}
