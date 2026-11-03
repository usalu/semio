//! ↩️ `change-built-up-min-radius` inverse: restores the previous `min_radius`.

use crate::change_built_up_min_radius::ChangeBuiltUpMinRadius;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpMinRadius, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_min_radius::ChangeBuiltUpMinRadius> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpMinRadius { id: payload.id.clone(), new_min_radius: row.min_radius })],
        None => Vec::new(),
    })
}
