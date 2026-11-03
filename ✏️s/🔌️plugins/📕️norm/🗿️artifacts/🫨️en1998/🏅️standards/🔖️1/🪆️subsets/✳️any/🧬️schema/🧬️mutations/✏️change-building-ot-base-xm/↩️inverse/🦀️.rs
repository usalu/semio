//! ↩️ `change-building-ot-base-xm` inverse: restores the previous `overturning_base_x_m`.

use crate::change_building_ot_base_x_m::ChangeBuildingOtBaseXM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingOtBaseXM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_ot_base_x_m::ChangeBuildingOtBaseXM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingOtBaseXM, base: &En1998Snapshot) -> Option<ChangeBuildingOtBaseXM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingOtBaseXM { id: payload.id.clone(), new_overturning_base_x_m: r0.overturning_base_x_m })
}
