//! ↩️ `change-member-label` inverse: restores the previous `label`.

use crate::change_member_label::ChangeMemberLabel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLabel, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_label::ChangeMemberLabel> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberLabel { id: payload.id.clone(), new_label: row.label.clone() })],
        None => Vec::new(),
    })
}
