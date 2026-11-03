//! ↩️ `change-dynamic-log-decrement` inverse: restores the previous `log_decrement`.

use crate::change_dynamic_log_decrement::ChangeDynamicLogDecrement;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDynamicLogDecrement, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dynamic_log_decrement::ChangeDynamicLogDecrement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDynamicLogDecrement, base: &En1991Snapshot) -> Option<ChangeDynamicLogDecrement> {
    let r0 = base.dynamic_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDynamicLogDecrement { id: payload.id.clone(), new_log_decrement: r0.log_decrement })
}
