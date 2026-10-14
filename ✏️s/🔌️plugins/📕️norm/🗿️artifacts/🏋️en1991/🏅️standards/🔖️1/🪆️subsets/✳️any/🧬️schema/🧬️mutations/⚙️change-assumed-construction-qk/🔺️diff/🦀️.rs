//! 🔺️ Diff for `change-assumed-construction-qk`.
use crate::change_assumed_construction_qk::ChangeAssumedConstructionQk;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedConstructionQk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_construction_qk == payload.new_assumed_construction_qk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_construction_qk: Some(payload.new_assumed_construction_qk), ..Default::default() })
}
