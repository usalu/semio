//! Diff for `change-depth`.
use super::ChangeDepth;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeDepth, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.depth == payload.new_depth {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { depth: Some(payload.new_depth), ..Default::default() })
}
