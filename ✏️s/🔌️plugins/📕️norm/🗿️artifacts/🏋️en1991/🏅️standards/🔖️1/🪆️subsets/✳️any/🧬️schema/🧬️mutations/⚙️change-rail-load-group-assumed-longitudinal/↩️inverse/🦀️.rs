//! ↩️ `change-rail-load-group-assumed-longitudinal` inverse: restores the previous `assumed_longitudinal`.

use crate::change_rail_load_group_assumed_longitudinal::ChangeRailLoadGroupAssumedLongitudinal;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailLoadGroupAssumedLongitudinal, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_load_group_assumed_longitudinal::ChangeRailLoadGroupAssumedLongitudinal> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailLoadGroupAssumedLongitudinal, base: &En1991Snapshot) -> Option<ChangeRailLoadGroupAssumedLongitudinal> {
    let r0 = base.rail_load_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailLoadGroupAssumedLongitudinal { id: payload.id.clone(), new_assumed_longitudinal: r0.assumed_longitudinal })
}
