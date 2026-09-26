//! Diff for `change-height`.
use super::ChangeHeight;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeHeight, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.height == payload.new_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { height: Some(payload.new_height), ..Default::default() })
}
