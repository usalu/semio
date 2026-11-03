//! ↩️ `change-rail-load-group-assumed-udl` inverse: restores the previous `assumed_udl`.

use crate::change_rail_load_group_assumed_udl::ChangeRailLoadGroupAssumedUdl;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailLoadGroupAssumedUdl, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_load_group_assumed_udl::ChangeRailLoadGroupAssumedUdl> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailLoadGroupAssumedUdl, base: &En1991Snapshot) -> Option<ChangeRailLoadGroupAssumedUdl> {
    let r0 = base.rail_load_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailLoadGroupAssumedUdl { id: payload.id.clone(), new_assumed_udl: r0.assumed_udl })
}
