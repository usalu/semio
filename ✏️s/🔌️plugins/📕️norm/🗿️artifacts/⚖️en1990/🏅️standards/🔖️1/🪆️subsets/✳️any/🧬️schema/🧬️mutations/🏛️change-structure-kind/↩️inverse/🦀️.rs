//! ↩️ `change-structure-kind` inverse: restores the previous `structure_kind`.

use crate::change_structure_kind::ChangeStructureKind;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeStructureKind, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_structure_kind::ChangeStructureKind> {
    let _ = payload;
    Ok(vec![Op::from(ChangeStructureKind { new_structure_kind: base.structure_kind })])
}
