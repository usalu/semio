//! 🔺️ Diff for `change-silo-bulk-density`.
use crate::change_silo_bulk_density::ChangeSiloBulkDensity;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloBulkDensity, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_bulk_density == payload.new_silo_bulk_density {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_bulk_density: Some(payload.new_silo_bulk_density), ..Default::default() })
}
