//! ↩️ `change-falling-assumed-force` inverse: restores the previous `assumed_force`.

use crate::change_falling_assumed_force::ChangeFallingAssumedForce;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFallingAssumedForce, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_falling_assumed_force::ChangeFallingAssumedForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFallingAssumedForce, base: &En1991Snapshot) -> Option<ChangeFallingAssumedForce> {
    let r0 = base.falling_loads.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFallingAssumedForce { id: payload.id.clone(), new_assumed_force: r0.assumed_force })
}
