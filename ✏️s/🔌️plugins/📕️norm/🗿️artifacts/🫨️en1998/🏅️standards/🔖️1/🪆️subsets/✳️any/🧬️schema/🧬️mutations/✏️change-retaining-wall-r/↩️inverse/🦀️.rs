//! ↩️ `change-retaining-wall-r` inverse: restores the previous `r`.

use crate::change_retaining_wall_r::ChangeRetainingWallR;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeRetainingWallR, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_retaining_wall_r::ChangeRetainingWallR> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRetainingWallR, base: &En1998Snapshot) -> Option<ChangeRetainingWallR> {
    let r0 = base.retaining_walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRetainingWallR { id: payload.id.clone(), new_r: r0.r })
}
