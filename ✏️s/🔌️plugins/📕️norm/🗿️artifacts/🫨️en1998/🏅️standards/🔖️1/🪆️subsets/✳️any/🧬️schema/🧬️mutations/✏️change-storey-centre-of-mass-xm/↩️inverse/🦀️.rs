//! ↩️ `change-storey-centre-of-mass-xm` inverse: restores the previous `centre_of_mass_x_m`.

use crate::change_storey_centre_of_mass_x_m::ChangeStoreyCentreOfMassXM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyCentreOfMassXM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_centre_of_mass_x_m::ChangeStoreyCentreOfMassXM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyCentreOfMassXM, base: &En1998Snapshot) -> Option<ChangeStoreyCentreOfMassXM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyCentreOfMassXM { building_id: payload.building_id.clone(), id: payload.id.clone(), new_centre_of_mass_x_m: r1.centre_of_mass_x_m })
}
