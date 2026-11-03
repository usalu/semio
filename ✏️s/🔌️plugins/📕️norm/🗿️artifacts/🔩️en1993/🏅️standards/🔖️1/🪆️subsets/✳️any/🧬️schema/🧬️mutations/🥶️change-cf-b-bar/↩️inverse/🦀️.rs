//! ↩️ `change-cf-b-bar` inverse: restores the previous `b_bar`.

use crate::change_cf_b_bar::ChangeCfBBar;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfBBar, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_b_bar::ChangeCfBBar> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfBBar { id: payload.id.clone(), new_b_bar: row.b_bar })],
        None => Vec::new(),
    })
}
