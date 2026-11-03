//! ↩️ `change-in-plane-moment` inverse: restores the previous `in_plane_moment_nm`.

use crate::change_in_plane_moment::ChangeInPlaneMoment;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeInPlaneMoment, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_in_plane_moment::ChangeInPlaneMoment> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeInPlaneMoment, base: &En1996Snapshot) -> Option<ChangeInPlaneMoment> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeInPlaneMoment { id: payload.id.clone(), new_in_plane_moment_nm: r0.in_plane_moment_nm })
}
