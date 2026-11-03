//! ↩️ `change-bridge-importance-class` inverse: restores the previous `importance_class`.

use crate::change_bridge_importance_class::ChangeBridgeImportanceClass;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeImportanceClass, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_importance_class::ChangeBridgeImportanceClass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeImportanceClass, base: &En1998Snapshot) -> Option<ChangeBridgeImportanceClass> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeImportanceClass { id: payload.id.clone(), new_importance_class: r0.importance_class })
}
