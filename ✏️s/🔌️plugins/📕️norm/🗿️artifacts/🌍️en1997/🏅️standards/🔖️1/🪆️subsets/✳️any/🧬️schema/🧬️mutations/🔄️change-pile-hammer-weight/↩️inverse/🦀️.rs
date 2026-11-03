//! ↩️ `change-pile-hammer-weight` inverse: restores the previous `hammer_weight`.

use crate::change_pile_hammer_weight::ChangePileHammerWeight;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileHammerWeight, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_hammer_weight::ChangePileHammerWeight> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileHammerWeight { id: payload.id.clone(), new_hammer_weight: row.hammer_weight })],
        None => Vec::new(),
    })
}
