//! Diff for `change-structure-kind`.
use super::ChangeStructureKind;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeStructureKind, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.new_structure_kind.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "invalid value", Vec::<String>::new());
    }
    if base.structure_kind == payload.new_structure_kind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { structure_kind: Some(payload.new_structure_kind.clone()), ..Default::default() })
}
