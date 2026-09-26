//! Diff for `change-silo-height`.
use super::ChangeSiloHeight;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeSiloHeight, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_height == payload.new_silo_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_height: Some(payload.new_silo_height), ..Default::default() })
}
