//! ↩️ `change-in-plane-shear` inverse: restores the previous `in_plane_shear_n`.

use crate::change_in_plane_shear::ChangeInPlaneShear;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeInPlaneShear, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_in_plane_shear::ChangeInPlaneShear> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeInPlaneShear, base: &En1996Snapshot) -> Option<ChangeInPlaneShear> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeInPlaneShear { id: payload.id.clone(), new_in_plane_shear_n: r0.in_plane_shear_n })
}
