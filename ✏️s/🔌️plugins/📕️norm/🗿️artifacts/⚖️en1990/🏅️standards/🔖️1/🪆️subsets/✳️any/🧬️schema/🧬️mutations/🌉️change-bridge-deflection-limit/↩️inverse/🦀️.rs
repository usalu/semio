//! ↩️ `change-bridge-deflection-limit` inverse: restores the previous `bridge_deflection_limit`.

use crate::change_bridge_deflection_limit::ChangeBridgeDeflectionLimit;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeflectionLimit, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deflection_limit::ChangeBridgeDeflectionLimit> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDeflectionLimit, base: &En1990Snapshot) -> Option<ChangeBridgeDeflectionLimit> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDeflectionLimit { id: payload.id.clone(), new_bridge_deflection_limit: r0.bridge_deflection_limit })
}
