//! ↩️ `change-cf-purlin-span` inverse: restores the previous `purlin_span`.

use crate::change_cf_purlin_span::ChangeCfPurlinSpan;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfPurlinSpan, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_purlin_span::ChangeCfPurlinSpan> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfPurlinSpan { id: payload.id.clone(), new_purlin_span: row.purlin_span })],
        None => Vec::new(),
    })
}
