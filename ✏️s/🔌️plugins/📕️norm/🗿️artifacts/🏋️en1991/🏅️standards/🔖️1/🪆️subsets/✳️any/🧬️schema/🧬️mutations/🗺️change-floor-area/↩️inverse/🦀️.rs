//! ↩️ `change-floor-area` inverse: restores the previous `area`.

use crate::change_floor_area::ChangeFloorArea;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFloorArea, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_area::ChangeFloorArea> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFloorArea, base: &En1991Snapshot) -> Option<ChangeFloorArea> {
    let r0 = base.floors.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFloorArea { id: payload.id.clone(), new_area: r0.area })
}
