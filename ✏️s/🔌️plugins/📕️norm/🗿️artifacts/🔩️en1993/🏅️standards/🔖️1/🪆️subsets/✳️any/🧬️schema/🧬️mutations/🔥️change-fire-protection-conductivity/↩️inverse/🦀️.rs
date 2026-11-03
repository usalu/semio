//! ↩️ `change-fire-protection-conductivity` inverse: restores the previous `protection_conductivity`.

use crate::change_fire_protection_conductivity::ChangeFireProtectionConductivity;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireProtectionConductivity, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_protection_conductivity::ChangeFireProtectionConductivity> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireProtectionConductivity { id: payload.id.clone(), new_protection_conductivity: row.protection_conductivity })],
        None => Vec::new(),
    })
}
