//! ↩️ `change-load-case-situation` inverse: restores the previous `design_situation`.

use crate::change_load_case_situation::ChangeLoadCaseSituation;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseSituation, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_situation::ChangeLoadCaseSituation> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLoadCaseSituation, base: &En1996Snapshot) -> Option<ChangeLoadCaseSituation> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLoadCaseSituation { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_design_situation: r1.design_situation })
}
