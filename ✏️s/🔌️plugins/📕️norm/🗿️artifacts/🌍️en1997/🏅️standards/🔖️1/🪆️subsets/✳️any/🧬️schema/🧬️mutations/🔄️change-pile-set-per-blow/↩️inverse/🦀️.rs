//! ↩️ `change-pile-set-per-blow` inverse: restores the previous `set_per_blow`.

use crate::change_pile_set_per_blow::ChangePileSetPerBlow;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileSetPerBlow, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_set_per_blow::ChangePileSetPerBlow> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileSetPerBlow { id: payload.id.clone(), new_set_per_blow: row.set_per_blow })],
        None => Vec::new(),
    })
}
