//! ↩️ `change-plated-equivalent-span` inverse: restores the previous `equivalent_span`.

use crate::change_plated_equivalent_span::ChangePlatedEquivalentSpan;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedEquivalentSpan, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_equivalent_span::ChangePlatedEquivalentSpan> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedEquivalentSpan { id: payload.id.clone(), new_equivalent_span: row.equivalent_span })],
        None => Vec::new(),
    })
}
