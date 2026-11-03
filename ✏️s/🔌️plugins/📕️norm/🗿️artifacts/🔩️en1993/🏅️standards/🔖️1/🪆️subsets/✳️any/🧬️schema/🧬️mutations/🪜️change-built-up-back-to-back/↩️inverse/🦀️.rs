//! ↩️ `change-built-up-back-to-back` inverse: restores the previous `back_to_back`.

use crate::change_built_up_back_to_back::ChangeBuiltUpBackToBack;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpBackToBack, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_back_to_back::ChangeBuiltUpBackToBack> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpBackToBack { id: payload.id.clone(), new_back_to_back: row.back_to_back })],
        None => Vec::new(),
    })
}
