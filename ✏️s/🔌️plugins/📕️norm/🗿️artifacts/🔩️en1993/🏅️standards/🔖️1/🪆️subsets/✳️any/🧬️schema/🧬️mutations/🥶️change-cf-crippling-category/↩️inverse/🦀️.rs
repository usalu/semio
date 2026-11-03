//! ↩️ `change-cf-crippling-category` inverse: restores the previous `crippling_category`.

use crate::change_cf_crippling_category::ChangeCfCripplingCategory;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfCripplingCategory, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_crippling_category::ChangeCfCripplingCategory> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfCripplingCategory { id: payload.id.clone(), new_crippling_category: row.crippling_category })],
        None => Vec::new(),
    })
}
