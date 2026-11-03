//! ↩️ `change-cf-support-length` inverse: restores the previous `support_length`.

use crate::change_cf_support_length::ChangeCfSupportLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfSupportLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_support_length::ChangeCfSupportLength> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfSupportLength { id: payload.id.clone(), new_support_length: row.support_length })],
        None => Vec::new(),
    })
}
