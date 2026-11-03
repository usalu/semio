//! ↩️ `change-plated-web-height` inverse: restores the previous `web_height`.

use crate::change_plated_web_height::ChangePlatedWebHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedWebHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_web_height::ChangePlatedWebHeight> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedWebHeight { id: payload.id.clone(), new_web_height: row.web_height })],
        None => Vec::new(),
    })
}
