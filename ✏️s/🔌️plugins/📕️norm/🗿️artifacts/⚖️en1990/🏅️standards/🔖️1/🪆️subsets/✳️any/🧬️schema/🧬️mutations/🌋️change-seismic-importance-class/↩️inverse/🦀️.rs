//! ↩️ `change-seismic-importance-class` inverse: restores the previous `importance_class`.

use crate::change_seismic_importance_class::ChangeSeismicImportanceClass;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeSeismicImportanceClass, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_seismic_importance_class::ChangeSeismicImportanceClass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSeismicImportanceClass, base: &En1990Snapshot) -> Option<ChangeSeismicImportanceClass> {
    let r0 = base.seismics.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSeismicImportanceClass { id: payload.id.clone(), new_importance_class: r0.importance_class })
}
