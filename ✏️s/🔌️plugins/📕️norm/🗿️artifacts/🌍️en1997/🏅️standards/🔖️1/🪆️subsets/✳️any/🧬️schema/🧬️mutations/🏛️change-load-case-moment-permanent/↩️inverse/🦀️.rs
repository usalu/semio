//! ↩️ `change-load-case-moment-permanent` inverse: restores the previous `moment_permanent`.

use crate::change_load_case_moment_permanent::ChangeLoadCaseMomentPermanent;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeLoadCaseMomentPermanent, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_moment_permanent::ChangeLoadCaseMomentPermanent> {
    Ok(match base.footings.iter().find(|row| row.id == payload.footing_id).and_then(|row| row.load_cases.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeLoadCaseMomentPermanent { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_moment_permanent: row.moment_permanent })],
        None => Vec::new(),
    })
}
