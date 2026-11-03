//! ↩️ `change-member-design-dimension` inverse: restores the previous `design_dimension`.

use crate::change_member_design_dimension::ChangeMemberDesignDimension;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDesignDimension, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_design_dimension::ChangeMemberDesignDimension> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberDesignDimension, base: &En1990Snapshot) -> Option<ChangeMemberDesignDimension> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberDesignDimension { id: payload.id.clone(), new_design_dimension: r0.design_dimension })
}
