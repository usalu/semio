//! 🔺️ Diff for `change-fire-occupancy`.
use crate::change_fire_occupancy::ChangeFireOccupancy;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeFireOccupancy, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_occupancy == payload.new_fire_occupancy {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_occupancy: Some(payload.new_fire_occupancy.clone()), ..Default::default() })
}
