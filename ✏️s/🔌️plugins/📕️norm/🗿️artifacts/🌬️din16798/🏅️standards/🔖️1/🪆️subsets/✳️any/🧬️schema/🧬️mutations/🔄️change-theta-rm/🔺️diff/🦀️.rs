//! 🔺️ `change-theta-rm` diff.
use super::ChangeThetaRm;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeThetaRm, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.theta_rm_c == payload.new_theta_rm_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { theta_rm_c: Some(payload.new_theta_rm_c), ..Default::default() })
}
