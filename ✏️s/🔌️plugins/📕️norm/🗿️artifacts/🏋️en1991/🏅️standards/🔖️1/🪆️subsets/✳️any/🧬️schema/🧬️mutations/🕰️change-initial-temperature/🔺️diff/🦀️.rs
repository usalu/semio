//! 🔺️ Diff for `change-initial-temperature`.
use crate::change_initial_temperature::ChangeInitialTemperature;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeInitialTemperature, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.t_0 == payload.new_t_0 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { t_0: Some(payload.new_t_0), ..Default::default() })
}
