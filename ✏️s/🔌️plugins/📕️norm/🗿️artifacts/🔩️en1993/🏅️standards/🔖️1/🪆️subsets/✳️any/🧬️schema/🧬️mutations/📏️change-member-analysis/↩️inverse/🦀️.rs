//! ↩️ `change-member-analysis` inverse: restores the previous `analysis`.

use crate::change_member_analysis::ChangeMemberAnalysis;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberAnalysis, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_analysis::ChangeMemberAnalysis> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberAnalysis { id: payload.id.clone(), new_analysis: row.analysis })],
        None => Vec::new(),
    })
}
