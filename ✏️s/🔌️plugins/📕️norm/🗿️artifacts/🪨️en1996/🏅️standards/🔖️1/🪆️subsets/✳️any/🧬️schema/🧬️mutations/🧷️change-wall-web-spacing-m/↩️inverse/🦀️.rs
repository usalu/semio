//! ↩️ `change-wall-web-spacing-m` inverse: restores the previous `web_spacing_m`.

use crate::change_wall_web_spacing_m::ChangeWallWebSpacingM;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeWallWebSpacingM, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_web_spacing_m::ChangeWallWebSpacingM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWallWebSpacingM, base: &En1996Snapshot) -> Option<ChangeWallWebSpacingM> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWallWebSpacingM { id: payload.id.clone(), new_web_spacing_m: r0.web_spacing_m })
}
