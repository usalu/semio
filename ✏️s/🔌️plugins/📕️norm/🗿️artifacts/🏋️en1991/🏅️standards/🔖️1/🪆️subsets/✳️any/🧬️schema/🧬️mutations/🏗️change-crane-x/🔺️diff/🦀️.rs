//! 🔺️ Diff for `change-crane-x`.
use crate::change_crane_x::ChangeCraneX;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeCraneX, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.crane_x == payload.new_crane_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_x: Some(payload.new_crane_x), ..Default::default() })
}
