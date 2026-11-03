//! ↩️ `change-member-material-conv-factor` inverse: restores the previous `material_conversion_factor`.

use crate::change_member_material_conv_factor::ChangeMemberMaterialConvFactor;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMaterialConvFactor, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_material_conv_factor::ChangeMemberMaterialConvFactor> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberMaterialConvFactor, base: &En1990Snapshot) -> Option<ChangeMemberMaterialConvFactor> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberMaterialConvFactor { id: payload.id.clone(), new_material_conversion_factor: r0.material_conversion_factor })
}
