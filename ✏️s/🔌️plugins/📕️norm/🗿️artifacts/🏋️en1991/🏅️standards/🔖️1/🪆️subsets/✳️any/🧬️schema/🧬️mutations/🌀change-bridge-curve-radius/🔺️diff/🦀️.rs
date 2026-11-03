//! 🔺️ Diff for `change-bridge-curve-radius`.
use crate::change_bridge_curve_radius::ChangeBridgeCurveRadius;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeBridgeCurveRadius, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_curve_radius == payload.new_bridge_curve_radius {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_curve_radius: Some(payload.new_bridge_curve_radius), ..Default::default() })
}
