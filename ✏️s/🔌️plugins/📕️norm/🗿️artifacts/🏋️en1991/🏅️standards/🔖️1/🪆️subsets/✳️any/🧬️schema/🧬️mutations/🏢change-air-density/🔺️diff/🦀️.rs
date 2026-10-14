//! 🔺️ Diff for `change-air-density`.
use crate::change_air_density::ChangeAirDensity;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAirDensity, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.air_density == payload.new_air_density {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { air_density: Some(payload.new_air_density), ..Default::default() })
}
