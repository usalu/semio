//! ↩️ `change-storey-centre-of-stiffness-ym` inverse: restores the previous `centre_of_stiffness_y_m`.

use crate::change_storey_centre_of_stiffness_y_m::ChangeStoreyCentreOfStiffnessYM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyCentreOfStiffnessYM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_centre_of_stiffness_y_m::ChangeStoreyCentreOfStiffnessYM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyCentreOfStiffnessYM, base: &En1998Snapshot) -> Option<ChangeStoreyCentreOfStiffnessYM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyCentreOfStiffnessYM { building_id: payload.building_id.clone(), id: payload.id.clone(), new_centre_of_stiffness_y_m: r1.centre_of_stiffness_y_m })
}
