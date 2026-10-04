//! Inverse for `change-structure-kind`.
use super::ChangeStructureKind;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(_payload: &ChangeStructureKind, base: &En1994Snapshot) -> Result<Vec<En1994Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1994Mutation::ChangeStructureKind(ChangeStructureKind { new_structure_kind: base.structure_kind.clone() })]

    })())
}
