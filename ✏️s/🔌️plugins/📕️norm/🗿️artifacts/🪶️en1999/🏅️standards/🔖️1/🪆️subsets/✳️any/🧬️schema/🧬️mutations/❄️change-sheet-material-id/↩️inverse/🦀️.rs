//! ↩️ `change-sheet-material-id` inverse: restores the previous `material_id`.

use crate::change_sheet_material_id::ChangeSheetMaterialId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSheetMaterialId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_sheet_material_id::ChangeSheetMaterialId> {
    Ok(match base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) {
        Some(row) => vec![Op::from(ChangeSheetMaterialId { cold_formed_id: payload.cold_formed_id.clone(), new_material_id: row.material_id.clone() })],
        None => Vec::new(),
    })
}
