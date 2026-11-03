//! ↩️ `change-cf-web-slope` inverse: restores the previous `web_slope`.

use crate::change_cf_web_slope::ChangeCfWebSlope;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfWebSlope, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_web_slope::ChangeCfWebSlope> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfWebSlope { id: payload.id.clone(), new_web_slope: row.web_slope })],
        None => Vec::new(),
    })
}
