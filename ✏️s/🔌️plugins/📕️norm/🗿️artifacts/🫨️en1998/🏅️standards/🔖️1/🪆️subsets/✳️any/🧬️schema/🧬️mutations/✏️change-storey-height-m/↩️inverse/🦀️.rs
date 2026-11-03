//! ↩️ `change-storey-height-m` inverse: restores the previous `height_m`.

use crate::change_storey_height_m::ChangeStoreyHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_height_m::ChangeStoreyHeightM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyHeightM, base: &En1998Snapshot) -> Option<ChangeStoreyHeightM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyHeightM { building_id: payload.building_id.clone(), id: payload.id.clone(), new_height_m: r1.height_m })
}
