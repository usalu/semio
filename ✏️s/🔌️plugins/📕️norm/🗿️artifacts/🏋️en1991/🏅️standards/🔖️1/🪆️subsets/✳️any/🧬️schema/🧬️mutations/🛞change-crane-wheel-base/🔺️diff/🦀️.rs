//! 🔺️ Diff for `change-crane-wheel-base`.
use crate::change_crane_wheel_base::ChangeCraneWheelBase;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeCraneWheelBase, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.crane_wheel_base == payload.new_crane_wheel_base {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_wheel_base: Some(payload.new_crane_wheel_base), ..Default::default() })
}
