//! ↩️ `change-rail-load-group-assumed-lateral` inverse: restores the previous `assumed_lateral`.

use crate::change_rail_load_group_assumed_lateral::ChangeRailLoadGroupAssumedLateral;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailLoadGroupAssumedLateral, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_load_group_assumed_lateral::ChangeRailLoadGroupAssumedLateral> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailLoadGroupAssumedLateral, base: &En1991Snapshot) -> Option<ChangeRailLoadGroupAssumedLateral> {
    let r0 = base.rail_load_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailLoadGroupAssumedLateral { id: payload.id.clone(), new_assumed_lateral: r0.assumed_lateral })
}
