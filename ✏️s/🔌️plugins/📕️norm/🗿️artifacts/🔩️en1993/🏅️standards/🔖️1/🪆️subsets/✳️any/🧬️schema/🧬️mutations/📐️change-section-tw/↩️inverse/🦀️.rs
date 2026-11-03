//! ↩️ `change-section-tw` inverse: restores the previous `tw`.

use crate::change_section_tw::ChangeSectionTw;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionTw, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_tw::ChangeSectionTw> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionTw { id: payload.id.clone(), new_tw: row.tw })],
        None => Vec::new(),
    })
}
