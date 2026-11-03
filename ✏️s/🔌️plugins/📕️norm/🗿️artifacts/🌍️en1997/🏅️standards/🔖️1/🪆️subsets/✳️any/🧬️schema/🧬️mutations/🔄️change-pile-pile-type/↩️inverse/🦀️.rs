//! ↩️ `change-pile-pile-type` inverse: restores the previous `pile_type`.

use crate::change_pile_pile_type::ChangePilePileType;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePilePileType, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_pile_type::ChangePilePileType> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePilePileType { id: payload.id.clone(), new_pile_type: row.pile_type })],
        None => Vec::new(),
    })
}
