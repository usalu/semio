//! ↩️ `change-retaining-wall-height-m` inverse: restores the previous `height_m`.

use crate::change_retaining_wall_height_m::ChangeRetainingWallHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeRetainingWallHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_retaining_wall_height_m::ChangeRetainingWallHeightM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRetainingWallHeightM, base: &En1998Snapshot) -> Option<ChangeRetainingWallHeightM> {
    let r0 = base.retaining_walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRetainingWallHeightM { id: payload.id.clone(), new_height_m: r0.height_m })
}
