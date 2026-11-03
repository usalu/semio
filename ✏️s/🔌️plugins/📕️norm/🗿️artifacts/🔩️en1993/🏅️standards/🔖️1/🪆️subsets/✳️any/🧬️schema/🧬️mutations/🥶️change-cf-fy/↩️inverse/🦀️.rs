//! ↩️ `change-cf-fy` inverse: restores the previous `fy`.

use crate::change_cf_fy::ChangeCfFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_fy::ChangeCfFy> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfFy { id: payload.id.clone(), new_fy: row.fy })],
        None => Vec::new(),
    })
}
