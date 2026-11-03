//! ↩️ `change-footing-length` inverse: restores the previous `length`.

use crate::change_footing_length::ChangeFootingLength;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeFootingLength, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_footing_length::ChangeFootingLength> {
    Ok(match base.footings.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFootingLength { id: payload.id.clone(), new_length: row.length })],
        None => Vec::new(),
    })
}
