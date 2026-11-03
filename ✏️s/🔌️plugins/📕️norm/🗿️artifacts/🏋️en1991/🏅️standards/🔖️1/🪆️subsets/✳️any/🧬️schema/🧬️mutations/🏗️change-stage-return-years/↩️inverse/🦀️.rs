//! ↩️ `change-stage-return-years` inverse: restores the previous `return_years`.

use crate::change_stage_return_years::ChangeStageReturnYears;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeStageReturnYears, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_stage_return_years::ChangeStageReturnYears> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStageReturnYears, base: &En1991Snapshot) -> Option<ChangeStageReturnYears> {
    let r0 = base.construction_stages.iter().find(|row| row.id == payload.id)?;
    Some(ChangeStageReturnYears { id: payload.id.clone(), new_return_years: r0.return_years })
}
