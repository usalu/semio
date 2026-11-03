//! ↩️ `change-dynamic-element-height` inverse: restores the previous `height`.

use crate::change_dynamic_element_height::ChangeDynamicElementHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDynamicElementHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dynamic_element_height::ChangeDynamicElementHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDynamicElementHeight, base: &En1991Snapshot) -> Option<ChangeDynamicElementHeight> {
    let r0 = base.dynamic_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDynamicElementHeight { id: payload.id.clone(), new_height: r0.height })
}
