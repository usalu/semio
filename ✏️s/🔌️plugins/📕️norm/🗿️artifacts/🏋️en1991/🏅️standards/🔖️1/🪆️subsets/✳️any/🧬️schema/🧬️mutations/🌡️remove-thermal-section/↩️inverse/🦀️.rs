//! ↩️ `remove-thermal-section` inverse: inserts the removed row back at its position.

use crate::remove_thermal_section::RemoveThermalSection;
use crate::insert_thermal_section::InsertThermalSection;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveThermalSection, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_thermal_section::InsertThermalSection> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveThermalSection, base: &En1991Snapshot) -> Option<InsertThermalSection> {
    let (index, row) = base.thermal_sections.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertThermalSection { index, item: row.clone() })
}
