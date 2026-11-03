//! ↩️ `change-built-up-batten-inertia` inverse: restores the previous `batten_inertia`.

use crate::change_built_up_batten_inertia::ChangeBuiltUpBattenInertia;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpBattenInertia, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_batten_inertia::ChangeBuiltUpBattenInertia> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpBattenInertia { id: payload.id.clone(), new_batten_inertia: row.batten_inertia })],
        None => Vec::new(),
    })
}
