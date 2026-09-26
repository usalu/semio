//! Diff for `change-silo-mu`.
use super::ChangeSiloMu;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeSiloMu, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_mu == payload.new_silo_mu {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_mu: Some(payload.new_silo_mu), ..Default::default() })
}
