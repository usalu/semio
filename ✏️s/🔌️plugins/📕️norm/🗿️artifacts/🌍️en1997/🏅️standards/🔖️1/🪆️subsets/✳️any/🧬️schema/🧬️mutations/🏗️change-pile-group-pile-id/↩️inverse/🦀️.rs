//! ↩️ `change-pile-group-pile-id` inverse: restores the previous `pile_id`.

use crate::change_pile_group_pile_id::ChangePileGroupPileId;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileGroupPileId, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_group_pile_id::ChangePileGroupPileId> {
    Ok(match base.pile_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileGroupPileId { id: payload.id.clone(), new_pile_id: row.pile_id.clone() })],
        None => Vec::new(),
    })
}
