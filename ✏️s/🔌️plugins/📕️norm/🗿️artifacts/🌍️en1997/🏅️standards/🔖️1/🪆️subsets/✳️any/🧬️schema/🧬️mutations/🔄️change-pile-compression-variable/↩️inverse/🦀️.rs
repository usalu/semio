//! ↩️ `change-pile-compression-variable` inverse: restores the previous `compression_variable`.

use crate::change_pile_compression_variable::ChangePileCompressionVariable;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileCompressionVariable, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_compression_variable::ChangePileCompressionVariable> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileCompressionVariable { id: payload.id.clone(), new_compression_variable: row.compression_variable })],
        None => Vec::new(),
    })
}
