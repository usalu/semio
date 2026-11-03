//! ↩️ `change-snow-guard-assumed-force` inverse: restores the previous `assumed_force`.

use crate::change_snow_guard_assumed_force::ChangeSnowGuardAssumedForce;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSnowGuardAssumedForce, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_snow_guard_assumed_force::ChangeSnowGuardAssumedForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSnowGuardAssumedForce, base: &En1991Snapshot) -> Option<ChangeSnowGuardAssumedForce> {
    let r0 = base.snow_guards.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSnowGuardAssumedForce { id: payload.id.clone(), new_assumed_force: r0.assumed_force })
}
