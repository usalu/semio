//! ↩️ `change-cf-thickness` inverse: restores the previous `thickness`.

use crate::change_cf_thickness::ChangeCfThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_thickness::ChangeCfThickness> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfThickness { id: payload.id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
