//! ↩️ `change-stage-assumed-qk` inverse: restores the previous `assumed_qk`.

use crate::change_stage_assumed_qk::ChangeStageAssumedQk;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeStageAssumedQk, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_stage_assumed_qk::ChangeStageAssumedQk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStageAssumedQk, base: &En1991Snapshot) -> Option<ChangeStageAssumedQk> {
    let r0 = base.construction_stages.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStageAssumedQk { id: payload.id.clone(), new_assumed_qk: r0.assumed_qk })
}
