//! 🔺️ `change-theta-rm` diff.
use crate::change_theta_rm::ChangeThetaRm;
use semio_s_artifact_norm_din16798_diff::Din16798Diff;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn diff(payload: &ChangeThetaRm, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.theta_rm_c == payload.new_theta_rm_c {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { theta_rm_c: Some(payload.new_theta_rm_c), ..Default::default() })
}
