//! Diff for `change-width`.
use super::ChangeWidth;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeWidth, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.width == payload.new_width {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { width: Some(payload.new_width), ..Default::default() })
}
