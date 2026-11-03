//! ↩️ `change-storey-label` inverse: restores the previous `label`.

use crate::change_storey_label::ChangeStoreyLabel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyLabel, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_label::ChangeStoreyLabel> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyLabel { id: payload.id.clone(), new_label: row.label.clone() })],
        None => Vec::new(),
    })
}
