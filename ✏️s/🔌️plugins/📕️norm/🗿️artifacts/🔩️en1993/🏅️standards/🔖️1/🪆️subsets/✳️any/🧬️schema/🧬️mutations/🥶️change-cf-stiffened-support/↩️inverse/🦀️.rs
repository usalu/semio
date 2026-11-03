//! ↩️ `change-cf-stiffened-support` inverse: restores the previous `stiffened_support`.

use crate::change_cf_stiffened_support::ChangeCfStiffenedSupport;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfStiffenedSupport, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_stiffened_support::ChangeCfStiffenedSupport> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfStiffenedSupport { id: payload.id.clone(), new_stiffened_support: row.stiffened_support })],
        None => Vec::new(),
    })
}
