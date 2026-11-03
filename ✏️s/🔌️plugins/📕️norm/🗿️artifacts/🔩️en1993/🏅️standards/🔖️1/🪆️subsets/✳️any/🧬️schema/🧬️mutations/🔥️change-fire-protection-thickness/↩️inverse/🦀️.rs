//! ↩️ `change-fire-protection-thickness` inverse: restores the previous `protection_thickness`.

use crate::change_fire_protection_thickness::ChangeFireProtectionThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireProtectionThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_protection_thickness::ChangeFireProtectionThickness> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireProtectionThickness { id: payload.id.clone(), new_protection_thickness: row.protection_thickness })],
        None => Vec::new(),
    })
}
