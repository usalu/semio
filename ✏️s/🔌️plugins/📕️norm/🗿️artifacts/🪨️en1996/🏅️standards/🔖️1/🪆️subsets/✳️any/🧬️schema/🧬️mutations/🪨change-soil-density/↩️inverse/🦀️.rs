//! ↩️ `change-soil-density` inverse: restores the previous `soil_density_kg_m3`.

use crate::change_soil_density::ChangeSoilDensity;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeSoilDensity, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_soil_density::ChangeSoilDensity> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSoilDensity, base: &En1996Snapshot) -> Option<ChangeSoilDensity> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSoilDensity { id: payload.id.clone(), new_soil_density_kg_m3: r0.soil_density_kg_m3 })
}
