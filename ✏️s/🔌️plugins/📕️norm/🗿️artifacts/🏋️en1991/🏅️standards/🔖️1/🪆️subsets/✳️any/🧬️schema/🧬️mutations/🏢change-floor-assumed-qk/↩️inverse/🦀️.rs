//! ↩️ `change-floor-assumed-qk` inverse: restores the previous `assumed_qk`.

use crate::change_floor_assumed_qk::ChangeFloorAssumedQk;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFloorAssumedQk, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_assumed_qk::ChangeFloorAssumedQk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFloorAssumedQk, base: &En1991Snapshot) -> Option<ChangeFloorAssumedQk> {
    let r0 = base.floors.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFloorAssumedQk { id: payload.id.clone(), new_assumed_qk: r0.assumed_qk })
}
