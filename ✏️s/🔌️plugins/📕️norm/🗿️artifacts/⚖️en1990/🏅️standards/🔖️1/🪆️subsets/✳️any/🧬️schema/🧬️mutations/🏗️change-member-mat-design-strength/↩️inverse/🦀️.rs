//! ↩️ `change-member-mat-design-strength` inverse: restores the previous `material_design_strength`.

use crate::change_member_mat_design_strength::ChangeMemberMatDesignStrength;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMatDesignStrength, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_mat_design_strength::ChangeMemberMatDesignStrength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberMatDesignStrength, base: &En1990Snapshot) -> Option<ChangeMemberMatDesignStrength> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberMatDesignStrength { id: payload.id.clone(), new_material_design_strength: r0.material_design_strength })
}
