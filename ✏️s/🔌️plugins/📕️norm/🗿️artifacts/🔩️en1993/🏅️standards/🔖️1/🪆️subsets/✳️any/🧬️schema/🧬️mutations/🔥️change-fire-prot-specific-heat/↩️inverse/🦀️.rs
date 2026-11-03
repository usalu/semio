//! ↩️ `change-fire-prot-specific-heat` inverse: restores the previous `protection_specific_heat`.

use crate::change_fire_prot_specific_heat::ChangeFireProtSpecificHeat;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireProtSpecificHeat, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_prot_specific_heat::ChangeFireProtSpecificHeat> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireProtSpecificHeat { id: payload.id.clone(), new_protection_specific_heat: row.protection_specific_heat })],
        None => Vec::new(),
    })
}
