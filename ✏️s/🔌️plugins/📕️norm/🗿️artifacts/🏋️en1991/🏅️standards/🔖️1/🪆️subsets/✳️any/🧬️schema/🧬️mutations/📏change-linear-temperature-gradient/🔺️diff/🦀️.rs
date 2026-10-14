//! 🔺️ Diff for `change-linear-temperature-gradient`.
use crate::change_linear_temperature_gradient::ChangeLinearTemperatureGradient;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeLinearTemperatureGradient, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.delta_t_m == payload.new_delta_t_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { delta_t_m: Some(payload.new_delta_t_m), ..Default::default() })
}
