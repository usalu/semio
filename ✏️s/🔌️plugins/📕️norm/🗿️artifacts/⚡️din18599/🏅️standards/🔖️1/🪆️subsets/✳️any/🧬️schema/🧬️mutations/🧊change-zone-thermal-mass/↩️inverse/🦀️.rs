//! ↩️ `change-zone-thermal-mass` inverse.

use crate::change_zone_thermal_mass::ChangeZoneThermalMass;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneThermalMass, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_thermal_mass::ChangeZoneThermalMass> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneThermalMass { zone_id: payload.zone_id.clone(), new_thermal_mass_class: row.thermal_mass_class, new_c_wirk_wh_m2k: row.c_wirk_wh_m2k })]).unwrap_or_default())
}
