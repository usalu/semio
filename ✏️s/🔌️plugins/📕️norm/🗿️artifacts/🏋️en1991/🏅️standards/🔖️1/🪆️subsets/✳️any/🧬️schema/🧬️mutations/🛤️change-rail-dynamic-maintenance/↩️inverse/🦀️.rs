//! ↩️ `change-rail-dynamic-maintenance` inverse: restores the previous `maintenance`.

use crate::change_rail_dynamic_maintenance::ChangeRailDynamicMaintenance;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailDynamicMaintenance, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_dynamic_maintenance::ChangeRailDynamicMaintenance> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailDynamicMaintenance, base: &En1991Snapshot) -> Option<ChangeRailDynamicMaintenance> {
    let r0 = base.rail_dynamics.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailDynamicMaintenance { id: payload.id.clone(), new_maintenance: r0.maintenance.clone() })
}
