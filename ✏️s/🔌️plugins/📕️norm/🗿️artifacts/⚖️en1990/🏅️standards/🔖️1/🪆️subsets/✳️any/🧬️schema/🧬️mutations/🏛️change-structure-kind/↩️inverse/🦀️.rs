//! ↩️ `change-structure-kind` inverse: restores the previous `structure_kind`.

use crate::change_structure_kind::ChangeStructureKind;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeStructureKind, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_structure_kind::ChangeStructureKind> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(_payload: &ChangeStructureKind, base: &En1990Snapshot) -> Option<crate::change_structure_kind::ChangeStructureKind> {
    Some(ChangeStructureKind { new_structure_kind: base.structure_kind })
}
