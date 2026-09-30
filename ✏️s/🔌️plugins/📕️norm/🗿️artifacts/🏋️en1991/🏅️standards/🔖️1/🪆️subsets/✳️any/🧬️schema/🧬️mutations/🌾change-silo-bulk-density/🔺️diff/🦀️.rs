//! Diff for `change-silo-bulk-density`.
use super::ChangeSiloBulkDensity;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeSiloBulkDensity, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_bulk_density == payload.new_silo_bulk_density {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_bulk_density: Some(payload.new_silo_bulk_density), ..Default::default() })
}
