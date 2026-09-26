//! Diff for `change-storey-count`.
use super::ChangeStoreyCount;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeStoreyCount, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.storey_count == payload.new_storey_count {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { storey_count: Some(payload.new_storey_count), ..Default::default() })
}
