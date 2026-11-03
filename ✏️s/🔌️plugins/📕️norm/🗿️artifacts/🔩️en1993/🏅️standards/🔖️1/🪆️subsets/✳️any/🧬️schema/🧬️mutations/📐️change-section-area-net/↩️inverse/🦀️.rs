//! ↩️ `change-section-area-net` inverse: restores the previous `area_net`.

use crate::change_section_area_net::ChangeSectionAreaNet;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionAreaNet, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_area_net::ChangeSectionAreaNet> {
    Ok(match base.sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSectionAreaNet { id: payload.id.clone(), new_area_net: row.area_net })],
        None => Vec::new(),
    })
}
