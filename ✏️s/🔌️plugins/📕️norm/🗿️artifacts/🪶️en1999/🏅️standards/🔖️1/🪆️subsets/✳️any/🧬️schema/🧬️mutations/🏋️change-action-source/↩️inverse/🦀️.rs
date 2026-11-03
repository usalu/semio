//! ↩️ `change-action-source` inverse: restores the previous `source`.

use crate::change_action_source::ChangeActionSource;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeActionSource, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_source::ChangeActionSource> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeActionSource { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_source: row.source })],
        None => Vec::new(),
    })
}
