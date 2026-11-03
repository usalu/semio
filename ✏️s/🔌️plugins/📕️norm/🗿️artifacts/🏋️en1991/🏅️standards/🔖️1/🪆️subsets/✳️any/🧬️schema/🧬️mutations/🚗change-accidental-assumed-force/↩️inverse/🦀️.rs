//! ↩️ `change-accidental-assumed-force` inverse: restores the previous `assumed_force`.

use crate::change_accidental_assumed_force::ChangeAccidentalAssumedForce;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeAccidentalAssumedForce, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accidental_assumed_force::ChangeAccidentalAssumedForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccidentalAssumedForce, base: &En1991Snapshot) -> Option<ChangeAccidentalAssumedForce> {
    let r0 = base.accidental_cases.iter().find(|row| row.id == payload.id)?;
    let current = r0.impact.first()?;
    Some(ChangeAccidentalAssumedForce { id: payload.id.clone(), new_assumed_force: current.assumed_force })
}
