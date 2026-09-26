//! Diff for `change-silo-kind`.
use super::ChangeSiloKind;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeSiloKind, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_kind == payload.new_silo_kind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_kind: Some(payload.new_silo_kind.clone()), ..Default::default() })
}
