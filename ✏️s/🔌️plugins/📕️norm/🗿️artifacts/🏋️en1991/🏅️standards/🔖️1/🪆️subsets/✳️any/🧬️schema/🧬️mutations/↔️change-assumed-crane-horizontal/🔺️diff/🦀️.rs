//! 🔺️ Diff for `change-assumed-crane-horizontal`.
use crate::change_assumed_crane_horizontal::ChangeAssumedCraneHorizontal;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedCraneHorizontal, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_crane_horizontal == payload.new_assumed_crane_horizontal {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_crane_horizontal: Some(payload.new_assumed_crane_horizontal), ..Default::default() })
}
