//! Diff for `change-silo-claimed`.
use super::ChangeSiloClaimed;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeSiloClaimed, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_claimed == payload.new_silo_claimed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_claimed: Some(payload.new_silo_claimed), ..Default::default() })
}
