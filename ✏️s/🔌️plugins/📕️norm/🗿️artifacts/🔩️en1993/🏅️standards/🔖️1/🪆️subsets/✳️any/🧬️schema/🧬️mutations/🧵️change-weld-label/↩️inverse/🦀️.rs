//! ↩️ `change-weld-label` inverse: restores the previous `label`.

use crate::change_weld_label::ChangeWeldLabel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldLabel, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_label::ChangeWeldLabel> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldLabel { id: payload.id.clone(), new_label: row.label.clone() })],
        None => Vec::new(),
    })
}
