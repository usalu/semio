//! ↩️ `change-cf-bending-moment` inverse: restores the previous `bending_moment`.

use crate::change_cf_bending_moment::ChangeCfBendingMoment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfBendingMoment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_bending_moment::ChangeCfBendingMoment> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfBendingMoment { id: payload.id.clone(), new_bending_moment: row.bending_moment })],
        None => Vec::new(),
    })
}
