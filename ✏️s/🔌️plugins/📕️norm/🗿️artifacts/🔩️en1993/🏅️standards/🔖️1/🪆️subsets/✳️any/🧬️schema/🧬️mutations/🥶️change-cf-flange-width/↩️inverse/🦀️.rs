//! ↩️ `change-cf-flange-width` inverse: restores the previous `flange_width`.

use crate::change_cf_flange_width::ChangeCfFlangeWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfFlangeWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_flange_width::ChangeCfFlangeWidth> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfFlangeWidth { id: payload.id.clone(), new_flange_width: row.flange_width })],
        None => Vec::new(),
    })
}
