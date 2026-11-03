//! ↩️ `change-cf-support-reaction` inverse: restores the previous `support_reaction`.

use crate::change_cf_support_reaction::ChangeCfSupportReaction;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfSupportReaction, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_support_reaction::ChangeCfSupportReaction> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfSupportReaction { id: payload.id.clone(), new_support_reaction: row.support_reaction })],
        None => Vec::new(),
    })
}
