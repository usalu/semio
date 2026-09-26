//! Diff for `change-assumed-silo-patch`.
use super::ChangeAssumedSiloPatch;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedSiloPatch, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_silo_patch == payload.new_assumed_silo_patch {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_silo_patch: Some(payload.new_assumed_silo_patch), ..Default::default() })
}
