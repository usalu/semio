//! Diff for `change-assumed-delta-t`.
use super::ChangeAssumedDeltaT;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedDeltaT, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_delta_t == payload.new_assumed_delta_t {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_delta_t: Some(payload.new_assumed_delta_t), ..Default::default() })
}
