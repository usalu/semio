//! ↩️ Inverse for `change-structure-kind`.
use crate::change_structure_kind::ChangeStructureKind;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(_payload: &ChangeStructureKind, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_structure_kind::ChangeStructureKind> {
    Ok((|| {
    vec![Op::from(ChangeStructureKind { new_structure_kind: base.structure_kind.clone() })]

    })())
}
