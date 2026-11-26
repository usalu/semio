//! ↩️ `change-inspection-level` inverse.

use crate::change_inspection_level::ChangeInspectionLevel;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeInspectionLevel, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_inspection_level::ChangeInspectionLevel> {
    Ok(match previous(mutation, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(_mutation: &ChangeInspectionLevel, base: &En1990Snapshot) -> Option<crate::change_inspection_level::ChangeInspectionLevel> {
    Some(ChangeInspectionLevel { new_inspection_level: base.inspection_level })
}
