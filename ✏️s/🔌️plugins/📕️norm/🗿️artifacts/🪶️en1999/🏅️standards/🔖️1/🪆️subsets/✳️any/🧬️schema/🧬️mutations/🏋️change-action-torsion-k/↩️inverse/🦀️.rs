//! ↩️ `change-action-torsion-k` inverse: restores the previous `torsion_k`.

use crate::change_action_torsion_k::ChangeActionTorsionK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeActionTorsionK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_torsion_k::ChangeActionTorsionK> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeActionTorsionK { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_torsion_k: row.torsion_k })],
        None => Vec::new(),
    })
}
