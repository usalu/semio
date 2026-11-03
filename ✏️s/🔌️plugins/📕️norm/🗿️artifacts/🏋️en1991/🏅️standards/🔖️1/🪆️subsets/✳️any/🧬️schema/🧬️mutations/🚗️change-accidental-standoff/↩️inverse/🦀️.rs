//! ↩️ `change-accidental-standoff` inverse: restores the previous `standoff`.

use crate::change_accidental_standoff::ChangeAccidentalStandoff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeAccidentalStandoff, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accidental_standoff::ChangeAccidentalStandoff> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccidentalStandoff, base: &En1991Snapshot) -> Option<ChangeAccidentalStandoff> {
    let r0 = base.accidental_cases.iter().find(|row| row.id == payload.id)?;
    let current = r0.explosion.first()?;
    Some(ChangeAccidentalStandoff { id: payload.id.clone(), new_standoff: current.standoff })
}
