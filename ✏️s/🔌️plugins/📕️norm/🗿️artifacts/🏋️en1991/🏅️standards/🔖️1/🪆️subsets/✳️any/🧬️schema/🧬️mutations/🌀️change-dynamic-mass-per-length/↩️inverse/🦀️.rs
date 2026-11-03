//! ↩️ `change-dynamic-mass-per-length` inverse: restores the previous `mass_per_length`.

use crate::change_dynamic_mass_per_length::ChangeDynamicMassPerLength;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDynamicMassPerLength, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dynamic_mass_per_length::ChangeDynamicMassPerLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDynamicMassPerLength, base: &En1991Snapshot) -> Option<ChangeDynamicMassPerLength> {
    let r0 = base.dynamic_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDynamicMassPerLength { id: payload.id.clone(), new_mass_per_length: r0.mass_per_length })
}
