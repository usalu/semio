//! ↩️ `change-member-use-fem` inverse: restores the previous `use_fem`.

use crate::change_member_use_fem::ChangeMemberUseFem;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberUseFem, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_use_fem::ChangeMemberUseFem> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberUseFem, base: &En1992Snapshot) -> Option<ChangeMemberUseFem> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    Some(ChangeMemberUseFem { member_id: payload.member_id.clone(), new_use_fem: r0.use_fem })
}
