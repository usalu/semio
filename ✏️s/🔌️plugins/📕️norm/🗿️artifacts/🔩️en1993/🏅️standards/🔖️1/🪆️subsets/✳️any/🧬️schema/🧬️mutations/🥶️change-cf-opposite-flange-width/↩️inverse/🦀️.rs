//! ↩️ `change-cf-opposite-flange-width` inverse: restores the previous `opposite_flange_width`.

use crate::change_cf_opposite_flange_width::ChangeCfOppositeFlangeWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfOppositeFlangeWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_opposite_flange_width::ChangeCfOppositeFlangeWidth> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfOppositeFlangeWidth { id: payload.id.clone(), new_opposite_flange_width: row.opposite_flange_width })],
        None => Vec::new(),
    })
}
