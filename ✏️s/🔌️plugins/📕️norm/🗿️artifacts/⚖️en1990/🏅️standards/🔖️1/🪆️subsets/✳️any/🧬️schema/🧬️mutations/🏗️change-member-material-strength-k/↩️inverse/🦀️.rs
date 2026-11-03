//! ↩️ `change-member-material-strength-k` inverse: restores the previous `material_strength_k`.

use crate::change_member_material_strength_k::ChangeMemberMaterialStrengthK;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMaterialStrengthK, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_material_strength_k::ChangeMemberMaterialStrengthK> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberMaterialStrengthK, base: &En1990Snapshot) -> Option<ChangeMemberMaterialStrengthK> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberMaterialStrengthK { id: payload.id.clone(), new_material_strength_k: r0.material_strength_k })
}
