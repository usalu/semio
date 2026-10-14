//! 🔺️ Diff for `change-wind-zone`.
use crate::change_wind_zone::ChangeWindZone;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeWindZone, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.wind_zone == payload.new_wind_zone {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_zone: Some(payload.new_wind_zone), ..Default::default() })
}
