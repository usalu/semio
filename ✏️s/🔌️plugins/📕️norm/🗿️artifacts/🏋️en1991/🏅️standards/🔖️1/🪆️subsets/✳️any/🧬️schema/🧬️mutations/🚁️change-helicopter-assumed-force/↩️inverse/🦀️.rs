//! ↩️ `change-helicopter-assumed-force` inverse: restores the previous `assumed_force`.

use crate::change_helicopter_assumed_force::ChangeHelicopterAssumedForce;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeHelicopterAssumedForce, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_helicopter_assumed_force::ChangeHelicopterAssumedForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeHelicopterAssumedForce, base: &En1991Snapshot) -> Option<ChangeHelicopterAssumedForce> {
    let r0 = base.helicopter_landings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeHelicopterAssumedForce { id: payload.id.clone(), new_assumed_force: r0.assumed_force })
}
