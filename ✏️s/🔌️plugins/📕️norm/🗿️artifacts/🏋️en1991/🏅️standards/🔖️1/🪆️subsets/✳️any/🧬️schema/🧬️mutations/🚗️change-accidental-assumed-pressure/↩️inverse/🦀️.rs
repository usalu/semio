//! ↩️ `change-accidental-assumed-pressure` inverse: restores the previous `assumed_pressure`.

use crate::change_accidental_assumed_pressure::ChangeAccidentalAssumedPressure;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeAccidentalAssumedPressure, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accidental_assumed_pressure::ChangeAccidentalAssumedPressure> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccidentalAssumedPressure, base: &En1991Snapshot) -> Option<ChangeAccidentalAssumedPressure> {
    let r0 = base.accidental_cases.iter().find(|row| row.id == payload.id)?;
    let current = r0.explosion.first()?;
    Some(ChangeAccidentalAssumedPressure { id: payload.id.clone(), new_assumed_pressure: current.assumed_pressure })
}
