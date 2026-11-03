//! ↩️ `change-bridge-method` inverse: restores the previous `method`.

use crate::change_bridge_method::ChangeBridgeMethod;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeMethod, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_method::ChangeBridgeMethod> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeMethod { id: payload.id.clone(), new_method: row.method })],
        None => Vec::new(),
    })
}
