//! Diff for `change-t-max`.
use super::ChangeTMax;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeTMax, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.t_max == payload.new_t_max {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { t_max: Some(payload.new_t_max), ..Default::default() })
}
