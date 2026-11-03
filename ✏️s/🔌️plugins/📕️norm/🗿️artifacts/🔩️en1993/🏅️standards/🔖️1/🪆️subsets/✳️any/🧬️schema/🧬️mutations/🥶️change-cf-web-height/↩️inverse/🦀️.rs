//! ↩️ `change-cf-web-height` inverse: restores the previous `web_height`.

use crate::change_cf_web_height::ChangeCfWebHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfWebHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_web_height::ChangeCfWebHeight> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfWebHeight { id: payload.id.clone(), new_web_height: row.web_height })],
        None => Vec::new(),
    })
}
