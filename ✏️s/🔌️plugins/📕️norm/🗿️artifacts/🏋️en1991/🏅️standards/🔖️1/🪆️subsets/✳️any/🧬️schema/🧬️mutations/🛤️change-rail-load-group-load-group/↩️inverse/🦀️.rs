//! ↩️ `change-rail-load-group-load-group` inverse: restores the previous `load_group`.

use crate::change_rail_load_group_load_group::ChangeRailLoadGroupLoadGroup;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailLoadGroupLoadGroup, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_load_group_load_group::ChangeRailLoadGroupLoadGroup> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailLoadGroupLoadGroup, base: &En1991Snapshot) -> Option<ChangeRailLoadGroupLoadGroup> {
    let r0 = base.rail_load_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailLoadGroupLoadGroup { id: payload.id.clone(), new_load_group: r0.load_group.clone() })
}
