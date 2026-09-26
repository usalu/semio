//! Diff for `change-structure-kind`.
use super::ChangeStructureKind;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeStructureKind, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.structure_kind == payload.new_structure_kind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { structure_kind: Some(payload.new_structure_kind), ..Default::default() })
}
