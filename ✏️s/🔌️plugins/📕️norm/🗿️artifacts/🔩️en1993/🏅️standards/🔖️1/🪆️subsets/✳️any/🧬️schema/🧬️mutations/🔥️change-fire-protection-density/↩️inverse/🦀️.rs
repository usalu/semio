//! ↩️ `change-fire-protection-density` inverse: restores the previous `protection_density`.

use crate::change_fire_protection_density::ChangeFireProtectionDensity;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireProtectionDensity, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_protection_density::ChangeFireProtectionDensity> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireProtectionDensity { id: payload.id.clone(), new_protection_density: row.protection_density })],
        None => Vec::new(),
    })
}
