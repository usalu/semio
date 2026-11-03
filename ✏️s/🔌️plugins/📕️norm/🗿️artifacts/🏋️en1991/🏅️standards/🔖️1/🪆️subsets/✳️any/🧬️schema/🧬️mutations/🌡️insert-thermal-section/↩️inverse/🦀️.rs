//! ↩️ `insert-thermal-section` inverse: removes the inserted row again.

use crate::insert_thermal_section::InsertThermalSection;
use crate::remove_thermal_section::RemoveThermalSection;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertThermalSection, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_thermal_section::RemoveThermalSection> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertThermalSection, base: &En1991Snapshot) -> Option<RemoveThermalSection> {
    Some(RemoveThermalSection { id: payload.item.id.clone() })
}
