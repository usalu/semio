//! ↩️ `change-member-mat-partial-factor` inverse: restores the previous `material_partial_factor`.

use crate::change_member_mat_partial_factor::ChangeMemberMatPartialFactor;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMatPartialFactor, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_mat_partial_factor::ChangeMemberMatPartialFactor> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberMatPartialFactor, base: &En1990Snapshot) -> Option<ChangeMemberMatPartialFactor> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberMatPartialFactor { id: payload.id.clone(), new_material_partial_factor: r0.material_partial_factor })
}
