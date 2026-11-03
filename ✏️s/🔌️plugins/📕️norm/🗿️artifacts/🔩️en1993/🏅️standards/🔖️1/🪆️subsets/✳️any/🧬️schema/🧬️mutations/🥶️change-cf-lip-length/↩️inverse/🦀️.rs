//! ↩️ `change-cf-lip-length` inverse: restores the previous `lip_length`.

use crate::change_cf_lip_length::ChangeCfLipLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfLipLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_lip_length::ChangeCfLipLength> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfLipLength { id: payload.id.clone(), new_lip_length: row.lip_length })],
        None => Vec::new(),
    })
}
