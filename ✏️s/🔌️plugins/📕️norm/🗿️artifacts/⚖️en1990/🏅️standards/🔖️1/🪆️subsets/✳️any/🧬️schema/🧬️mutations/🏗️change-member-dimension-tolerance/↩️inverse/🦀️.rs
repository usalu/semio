//! ↩️ `change-member-dimension-tolerance` inverse: restores the previous `dimension_tolerance`.

use crate::change_member_dimension_tolerance::ChangeMemberDimensionTolerance;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDimensionTolerance, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_dimension_tolerance::ChangeMemberDimensionTolerance> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberDimensionTolerance, base: &En1990Snapshot) -> Option<ChangeMemberDimensionTolerance> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberDimensionTolerance { id: payload.id.clone(), new_dimension_tolerance: r0.dimension_tolerance })
}
