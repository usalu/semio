//! Diff for `change-assumed-qf-d`.
use super::ChangeAssumedQfD;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedQfD, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_qf_d == payload.new_assumed_qf_d {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_qf_d: Some(payload.new_assumed_qf_d), ..Default::default() })
}
