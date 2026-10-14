//! 🔺️ Diff for `change-hoisting-speed`.
use crate::change_hoisting_speed::ChangeHoistingSpeed;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeHoistingSpeed, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.hoisting_speed == payload.new_hoisting_speed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { hoisting_speed: Some(payload.new_hoisting_speed), ..Default::default() })
}
