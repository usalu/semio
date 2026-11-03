//! ↩️ `change-building-ot-base-ym` inverse: restores the previous `overturning_base_y_m`.

use crate::change_building_ot_base_y_m::ChangeBuildingOtBaseYM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingOtBaseYM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_ot_base_y_m::ChangeBuildingOtBaseYM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingOtBaseYM, base: &En1998Snapshot) -> Option<ChangeBuildingOtBaseYM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingOtBaseYM { id: payload.id.clone(), new_overturning_base_y_m: r0.overturning_base_y_m })
}
