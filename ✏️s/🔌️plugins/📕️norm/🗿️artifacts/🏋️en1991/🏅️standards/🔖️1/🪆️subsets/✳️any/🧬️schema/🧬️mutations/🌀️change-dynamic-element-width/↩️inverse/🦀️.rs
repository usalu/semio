//! ↩️ `change-dynamic-element-width` inverse: restores the previous `width`.

use crate::change_dynamic_element_width::ChangeDynamicElementWidth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDynamicElementWidth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dynamic_element_width::ChangeDynamicElementWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDynamicElementWidth, base: &En1991Snapshot) -> Option<ChangeDynamicElementWidth> {
    let r0 = base.dynamic_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDynamicElementWidth { id: payload.id.clone(), new_width: r0.width })
}
