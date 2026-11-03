//! ↩️ `change-bridge-deflection` inverse: restores the previous `bridge_deflection`.

use crate::change_bridge_deflection::ChangeBridgeDeflection;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeflection, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deflection::ChangeBridgeDeflection> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDeflection, base: &En1990Snapshot) -> Option<ChangeBridgeDeflection> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDeflection { id: payload.id.clone(), new_bridge_deflection: r0.bridge_deflection })
}
