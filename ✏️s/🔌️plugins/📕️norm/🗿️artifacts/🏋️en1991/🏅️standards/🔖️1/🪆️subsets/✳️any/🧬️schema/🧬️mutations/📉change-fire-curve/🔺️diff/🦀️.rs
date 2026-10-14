//! 🔺️ Diff for `change-fire-curve`.
use crate::change_fire_curve::ChangeFireCurve;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeFireCurve, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_curve == payload.new_fire_curve {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_curve: Some(payload.new_fire_curve), ..Default::default() })
}
