//! ↩️ `change-traffic-group-variable-id` inverse: restores the previous `variable_id`.

use crate::change_traffic_group_variable_id::ChangeTrafficGroupVariableId;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTrafficGroupVariableId, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_traffic_group_variable_id::ChangeTrafficGroupVariableId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTrafficGroupVariableId, base: &En1990Snapshot) -> Option<ChangeTrafficGroupVariableId> {
    let r0 = base.traffic_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTrafficGroupVariableId { id: payload.id.clone(), new_variable_id: r0.variable_id.clone() })
}
