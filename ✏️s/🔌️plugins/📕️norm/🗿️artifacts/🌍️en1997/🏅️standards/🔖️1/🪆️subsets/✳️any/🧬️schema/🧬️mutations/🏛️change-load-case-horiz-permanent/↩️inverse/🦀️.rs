//! ↩️ `change-load-case-horiz-permanent` inverse: restores the previous `horizontal_permanent`.

use crate::change_load_case_horiz_permanent::ChangeLoadCaseHorizPermanent;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseHorizPermanent, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_horiz_permanent::ChangeLoadCaseHorizPermanent> {
    Ok(match base.footings.iter().find(|row| row.id == payload.footing_id).and_then(|row| row.load_cases.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeLoadCaseHorizPermanent { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_horizontal_permanent: row.horizontal_permanent })],
        None => Vec::new(),
    })
}
