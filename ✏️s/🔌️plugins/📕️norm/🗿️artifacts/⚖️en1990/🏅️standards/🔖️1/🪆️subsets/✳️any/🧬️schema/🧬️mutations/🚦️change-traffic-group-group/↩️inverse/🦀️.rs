//! ↩️ `change-traffic-group-group` inverse: restores the previous `group`.

use crate::change_traffic_group_group::ChangeTrafficGroupGroup;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTrafficGroupGroup, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_traffic_group_group::ChangeTrafficGroupGroup> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTrafficGroupGroup, base: &En1990Snapshot) -> Option<ChangeTrafficGroupGroup> {
    let r0 = base.traffic_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTrafficGroupGroup { id: payload.id.clone(), new_group: r0.group })
}
