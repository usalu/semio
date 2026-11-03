//! ↩️ `change-rail-load-group-assumed-axle-load` inverse: restores the previous `assumed_axle_load`.

use crate::change_rail_load_group_assumed_axle_load::ChangeRailLoadGroupAssumedAxleLoad;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailLoadGroupAssumedAxleLoad, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_load_group_assumed_axle_load::ChangeRailLoadGroupAssumedAxleLoad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailLoadGroupAssumedAxleLoad, base: &En1991Snapshot) -> Option<ChangeRailLoadGroupAssumedAxleLoad> {
    let r0 = base.rail_load_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailLoadGroupAssumedAxleLoad { id: payload.id.clone(), new_assumed_axle_load: r0.assumed_axle_load })
}
