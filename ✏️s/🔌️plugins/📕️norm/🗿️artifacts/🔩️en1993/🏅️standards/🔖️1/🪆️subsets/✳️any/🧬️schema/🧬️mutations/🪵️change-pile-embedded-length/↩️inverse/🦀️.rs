//! ↩️ `change-pile-embedded-length` inverse: restores the previous `embedded_length`.

use crate::change_pile_embedded_length::ChangePileEmbeddedLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileEmbeddedLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_embedded_length::ChangePileEmbeddedLength> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileEmbeddedLength { id: payload.id.clone(), new_embedded_length: row.embedded_length })],
        None => Vec::new(),
    })
}
