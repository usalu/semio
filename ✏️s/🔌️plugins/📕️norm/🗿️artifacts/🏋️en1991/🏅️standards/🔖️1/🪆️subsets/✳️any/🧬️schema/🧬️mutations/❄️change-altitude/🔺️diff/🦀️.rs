//! 🔺️ Diff for `change-altitude`.
use crate::change_altitude::ChangeAltitude;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAltitude, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.altitude == payload.new_altitude {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { altitude: Some(payload.new_altitude), ..Default::default() })
}
