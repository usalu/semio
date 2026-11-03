//! ↩️ `change-stage-activity` inverse: restores the previous `activity`.

use crate::change_stage_activity::ChangeStageActivity;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeStageActivity, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_stage_activity::ChangeStageActivity> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStageActivity, base: &En1991Snapshot) -> Option<ChangeStageActivity> {
    let r0 = base.construction_stages.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStageActivity { id: payload.id.clone(), new_activity: r0.activity.clone() })
}
