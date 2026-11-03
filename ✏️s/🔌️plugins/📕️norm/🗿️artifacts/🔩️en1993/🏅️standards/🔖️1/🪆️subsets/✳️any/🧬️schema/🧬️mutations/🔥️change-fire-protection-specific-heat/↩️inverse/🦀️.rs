//! ↩️ `change-fire-protection-specific-heat` inverse: restores the previous `protection_specific_heat`.

use crate::change_fire_protection_specific_heat::ChangeFireProtectionSpecificHeat;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireProtectionSpecificHeat, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_protection_specific_heat::ChangeFireProtectionSpecificHeat> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireProtectionSpecificHeat { id: payload.id.clone(), new_protection_specific_heat: row.protection_specific_heat })],
        None => Vec::new(),
    })
}
