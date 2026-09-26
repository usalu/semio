//! Diff for `change-t-min`.
use super::ChangeTMin;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeTMin, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.t_min == payload.new_t_min {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { t_min: Some(payload.new_t_min), ..Default::default() })
}
