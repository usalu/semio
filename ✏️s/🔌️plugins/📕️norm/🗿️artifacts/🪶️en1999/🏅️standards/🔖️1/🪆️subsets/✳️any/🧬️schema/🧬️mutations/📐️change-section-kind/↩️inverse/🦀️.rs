//! ↩️ `change-section-kind` inverse: restores the previous `kind`.

use crate::change_section_kind::ChangeSectionKind;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionKind, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_kind::ChangeSectionKind> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id) {
        Some(row) => vec![Op::from(ChangeSectionKind { section_id: payload.section_id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
