//! ↩️ `change-stage-duration-days` inverse: restores the previous `duration_days`.

use crate::change_stage_duration_days::ChangeStageDurationDays;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeStageDurationDays, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_stage_duration_days::ChangeStageDurationDays> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStageDurationDays, base: &En1991Snapshot) -> Option<ChangeStageDurationDays> {
    let r0 = base.construction_stages.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStageDurationDays { id: payload.id.clone(), new_duration_days: r0.duration_days })
}
