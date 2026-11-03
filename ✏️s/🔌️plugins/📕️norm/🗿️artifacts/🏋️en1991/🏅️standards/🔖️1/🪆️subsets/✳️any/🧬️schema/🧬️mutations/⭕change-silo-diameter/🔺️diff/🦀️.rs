//! 🔺️ Diff for `change-silo-diameter`.
use crate::change_silo_diameter::ChangeSiloDiameter;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloDiameter, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_diameter == payload.new_silo_diameter {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_diameter: Some(payload.new_silo_diameter), ..Default::default() })
}
