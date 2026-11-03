//! ↩️ `change-pile-compression-permanent` inverse: restores the previous `compression_permanent`.

use crate::change_pile_compression_permanent::ChangePileCompressionPermanent;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileCompressionPermanent, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_compression_permanent::ChangePileCompressionPermanent> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileCompressionPermanent { id: payload.id.clone(), new_compression_permanent: row.compression_permanent })],
        None => Vec::new(),
    })
}
