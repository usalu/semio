//! ↩️ `change-built-up-planes` inverse: restores the previous `planes`.

use crate::change_built_up_planes::ChangeBuiltUpPlanes;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpPlanes, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_planes::ChangeBuiltUpPlanes> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpPlanes { id: payload.id.clone(), new_planes: row.planes })],
        None => Vec::new(),
    })
}
