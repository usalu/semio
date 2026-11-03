//! ↩️ `change-section-it` inverse: restores the previous `it`.

use crate::change_section_it::ChangeSectionIt;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionIt, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_it::ChangeSectionIt> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionIt { id: payload.id.clone(), new_it: row.it })],
        None => Vec::new(),
    })
}
