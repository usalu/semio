//! ↩️ `change-built-up-label` inverse: restores the previous `label`.

use crate::change_built_up_label::ChangeBuiltUpLabel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpLabel, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_label::ChangeBuiltUpLabel> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpLabel { id: payload.id.clone(), new_label: row.label.clone() })],
        None => Vec::new(),
    })
}
