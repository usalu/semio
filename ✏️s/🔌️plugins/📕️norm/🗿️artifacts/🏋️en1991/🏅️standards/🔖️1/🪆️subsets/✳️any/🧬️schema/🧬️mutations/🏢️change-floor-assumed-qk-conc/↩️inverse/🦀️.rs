//! ↩️ `change-floor-assumed-qk-conc` inverse: restores the previous `assumed_qk_concentrated`.

use crate::change_floor_assumed_qk_conc::ChangeFloorAssumedQkConc;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFloorAssumedQkConc, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_assumed_qk_conc::ChangeFloorAssumedQkConc> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFloorAssumedQkConc, base: &En1991Snapshot) -> Option<ChangeFloorAssumedQkConc> {
    let r0 = base.floors.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFloorAssumedQkConc { id: payload.id.clone(), new_assumed_qk_concentrated: r0.assumed_qk_concentrated })
}
