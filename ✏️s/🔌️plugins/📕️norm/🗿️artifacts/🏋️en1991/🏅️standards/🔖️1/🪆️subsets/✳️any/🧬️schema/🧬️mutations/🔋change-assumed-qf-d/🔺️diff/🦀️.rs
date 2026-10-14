//! 🔺️ Diff for `change-assumed-qf-d`.
use crate::change_assumed_qf_d::ChangeAssumedQfD;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedQfD, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_qf_d == payload.new_assumed_qf_d {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_qf_d: Some(payload.new_assumed_qf_d), ..Default::default() })
}
