//! Inverse for `change-structure-kind`.
use super::ChangeStructureKind;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeStructureKind, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeStructureKind(ChangeStructureKind { new_structure_kind: base.structure_kind })]
}
