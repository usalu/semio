//! ↩️ `change-section-tf` inverse: restores the previous `tf`.

use crate::change_section_tf::ChangeSectionTf;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionTf, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_tf::ChangeSectionTf> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionTf { id: payload.id.clone(), new_tf: row.tf })],
        None => Vec::new(),
    })
}
