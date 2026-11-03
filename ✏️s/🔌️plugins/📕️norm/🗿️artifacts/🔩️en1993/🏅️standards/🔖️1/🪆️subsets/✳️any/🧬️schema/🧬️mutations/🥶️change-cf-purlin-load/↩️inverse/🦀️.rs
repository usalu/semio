//! ↩️ `change-cf-purlin-load` inverse: restores the previous `purlin_load`.

use crate::change_cf_purlin_load::ChangeCfPurlinLoad;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfPurlinLoad, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_purlin_load::ChangeCfPurlinLoad> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfPurlinLoad { id: payload.id.clone(), new_purlin_load: row.purlin_load })],
        None => Vec::new(),
    })
}
