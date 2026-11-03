//! 🔺️ Diff for `change-crane-runway-length`.
use crate::change_crane_runway_length::ChangeCraneRunwayLength;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeCraneRunwayLength, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.crane_runway_length == payload.new_crane_runway_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_runway_length: Some(payload.new_crane_runway_length), ..Default::default() })
}
