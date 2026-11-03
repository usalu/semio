//! ↩️ `change-dynamic-first-frequency` inverse: restores the previous `first_frequency`.

use crate::change_dynamic_first_frequency::ChangeDynamicFirstFrequency;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDynamicFirstFrequency, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dynamic_first_frequency::ChangeDynamicFirstFrequency> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDynamicFirstFrequency, base: &En1991Snapshot) -> Option<ChangeDynamicFirstFrequency> {
    let r0 = base.dynamic_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDynamicFirstFrequency { id: payload.id.clone(), new_first_frequency: r0.first_frequency })
}
