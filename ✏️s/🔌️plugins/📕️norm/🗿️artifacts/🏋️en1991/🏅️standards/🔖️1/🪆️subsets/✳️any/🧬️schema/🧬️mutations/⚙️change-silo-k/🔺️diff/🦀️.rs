//! Diff for `change-silo-k`.
use super::ChangeSiloK;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeSiloK, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_k == payload.new_silo_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_k: Some(payload.new_silo_k), ..Default::default() })
}
