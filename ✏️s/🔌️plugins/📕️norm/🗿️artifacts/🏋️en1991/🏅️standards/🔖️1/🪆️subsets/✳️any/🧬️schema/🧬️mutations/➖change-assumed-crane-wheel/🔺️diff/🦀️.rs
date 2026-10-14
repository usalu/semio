//! 🔺️ Diff for `change-assumed-crane-wheel`.
use crate::change_assumed_crane_wheel::ChangeAssumedCraneWheel;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedCraneWheel, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_crane_wheel == payload.new_assumed_crane_wheel {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_crane_wheel: Some(payload.new_assumed_crane_wheel), ..Default::default() })
}
