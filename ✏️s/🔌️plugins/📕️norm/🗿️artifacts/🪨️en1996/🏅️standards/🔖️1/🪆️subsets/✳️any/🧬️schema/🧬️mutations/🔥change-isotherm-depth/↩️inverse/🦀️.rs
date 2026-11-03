//! ↩️ `change-isotherm-depth` inverse: restores the previous `isotherm_depth_m`.

use crate::change_isotherm_depth::ChangeIsothermDepth;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeIsothermDepth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_isotherm_depth::ChangeIsothermDepth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeIsothermDepth, base: &En1996Snapshot) -> Option<ChangeIsothermDepth> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeIsothermDepth { id: payload.id.clone(), new_isotherm_depth_m: r0.isotherm_depth_m })
}
