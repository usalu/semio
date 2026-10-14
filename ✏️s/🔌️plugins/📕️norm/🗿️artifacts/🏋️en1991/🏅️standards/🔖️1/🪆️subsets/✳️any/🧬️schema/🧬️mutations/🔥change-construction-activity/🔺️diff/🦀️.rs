//! 🔺️ Diff for `change-construction-activity`.
use crate::change_construction_activity::ChangeConstructionActivity;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeConstructionActivity, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.construction_activity == payload.new_construction_activity {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_activity: Some(payload.new_construction_activity.clone()), ..Default::default() })
}
