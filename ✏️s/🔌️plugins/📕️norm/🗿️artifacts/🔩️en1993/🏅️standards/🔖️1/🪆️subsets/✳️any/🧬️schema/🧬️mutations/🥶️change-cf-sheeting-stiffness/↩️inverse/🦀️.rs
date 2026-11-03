//! ↩️ `change-cf-sheeting-stiffness` inverse: restores the previous `sheeting_stiffness`.

use crate::change_cf_sheeting_stiffness::ChangeCfSheetingStiffness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfSheetingStiffness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_sheeting_stiffness::ChangeCfSheetingStiffness> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfSheetingStiffness { id: payload.id.clone(), new_sheeting_stiffness: row.sheeting_stiffness })],
        None => Vec::new(),
    })
}
