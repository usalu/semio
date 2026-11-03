//! ↩️ `change-member-nominal-dimension` inverse: restores the previous `nominal_dimension`.

use crate::change_member_nominal_dimension::ChangeMemberNominalDimension;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberNominalDimension, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_nominal_dimension::ChangeMemberNominalDimension> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberNominalDimension, base: &En1990Snapshot) -> Option<ChangeMemberNominalDimension> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberNominalDimension { id: payload.id.clone(), new_nominal_dimension: r0.nominal_dimension })
}
