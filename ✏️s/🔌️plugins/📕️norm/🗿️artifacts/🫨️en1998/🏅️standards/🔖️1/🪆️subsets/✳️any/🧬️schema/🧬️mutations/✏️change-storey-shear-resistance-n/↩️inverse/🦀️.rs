//! ↩️ `change-storey-shear-resistance-n` inverse: restores the previous `shear_resistance_n`.

use crate::change_storey_shear_resistance_n::ChangeStoreyShearResistanceN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyShearResistanceN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_shear_resistance_n::ChangeStoreyShearResistanceN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStoreyShearResistanceN, base: &En1998Snapshot) -> Option<ChangeStoreyShearResistanceN> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.storeys.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStoreyShearResistanceN { building_id: payload.building_id.clone(), id: payload.id.clone(), new_shear_resistance_n: r1.shear_resistance_n })
}
