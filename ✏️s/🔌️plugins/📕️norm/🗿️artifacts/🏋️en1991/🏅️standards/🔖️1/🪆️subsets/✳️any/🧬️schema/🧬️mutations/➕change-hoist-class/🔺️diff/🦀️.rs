//! 🔺️ Diff for `change-hoist-class`.
use crate::change_hoist_class::ChangeHoistClass;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeHoistClass, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.hoist_class == payload.new_hoist_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { hoist_class: Some(payload.new_hoist_class.clone()), ..Default::default() })
}
