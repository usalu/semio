//! 🔺️ Diff for `change-crane-claimed`.
use crate::change_crane_claimed::ChangeCraneClaimed;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeCraneClaimed, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.crane_claimed == payload.new_crane_claimed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_claimed: Some(payload.new_crane_claimed), ..Default::default() })
}
