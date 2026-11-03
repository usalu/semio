//! ↩️ `change-section-kind` inverse: restores the previous `kind`.

use crate::change_section_kind::ChangeSectionKind;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionKind, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_kind::ChangeSectionKind> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
