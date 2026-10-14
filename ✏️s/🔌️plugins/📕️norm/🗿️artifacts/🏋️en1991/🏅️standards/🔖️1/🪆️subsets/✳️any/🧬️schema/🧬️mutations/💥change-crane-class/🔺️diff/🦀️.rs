//! 🔺️ Diff for `change-crane-class`.
use crate::change_crane_class::ChangeCraneClass;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeCraneClass, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.crane_class == payload.new_crane_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_class: Some(payload.new_crane_class.clone()), ..Default::default() })
}
