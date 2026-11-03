//! ↩️ `change-load-case-design-situation` inverse: restores the previous `design_situation`.

use crate::change_load_case_design_situation::ChangeLoadCaseDesignSituation;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseDesignSituation, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_design_situation::ChangeLoadCaseDesignSituation> {
    Ok(match base.footings.iter().find(|row| row.id == payload.footing_id).and_then(|row| row.load_cases.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeLoadCaseDesignSituation { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_design_situation: row.design_situation })],
        None => Vec::new(),
    })
}
