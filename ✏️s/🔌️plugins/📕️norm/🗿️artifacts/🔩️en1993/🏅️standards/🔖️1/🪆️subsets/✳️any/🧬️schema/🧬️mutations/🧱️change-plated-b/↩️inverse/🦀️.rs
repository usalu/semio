//! ↩️ `change-plated-b` inverse: restores the previous `b`.

use crate::change_plated_b::ChangePlatedB;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedB, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_b::ChangePlatedB> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedB { id: payload.id.clone(), new_b: row.b })],
        None => Vec::new(),
    })
}
