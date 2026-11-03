//! ↩️ `change-dynamic-element-strouhal` inverse: restores the previous `strouhal`.

use crate::change_dynamic_element_strouhal::ChangeDynamicElementStrouhal;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDynamicElementStrouhal, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dynamic_element_strouhal::ChangeDynamicElementStrouhal> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDynamicElementStrouhal, base: &En1991Snapshot) -> Option<ChangeDynamicElementStrouhal> {
    let r0 = base.dynamic_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDynamicElementStrouhal { id: payload.id.clone(), new_strouhal: r0.strouhal })
}
