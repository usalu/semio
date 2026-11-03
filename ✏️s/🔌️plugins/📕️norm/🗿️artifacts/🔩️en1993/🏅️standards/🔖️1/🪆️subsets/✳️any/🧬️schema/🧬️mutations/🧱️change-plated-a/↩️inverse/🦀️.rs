//! ↩️ `change-plated-a` inverse: restores the previous `a`.

use crate::change_plated_a::ChangePlatedA;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedA, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_a::ChangePlatedA> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedA { id: payload.id.clone(), new_a: row.a })],
        None => Vec::new(),
    })
}
