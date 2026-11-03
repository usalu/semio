//! ↩️ `change-silo-fabrication-class` inverse: restores the previous `fabrication_class`.

use crate::change_silo_fabrication_class::ChangeSiloFabricationClass;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloFabricationClass, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_fabrication_class::ChangeSiloFabricationClass> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloFabricationClass { id: payload.id.clone(), new_fabrication_class: row.fabrication_class })],
        None => Vec::new(),
    })
}
