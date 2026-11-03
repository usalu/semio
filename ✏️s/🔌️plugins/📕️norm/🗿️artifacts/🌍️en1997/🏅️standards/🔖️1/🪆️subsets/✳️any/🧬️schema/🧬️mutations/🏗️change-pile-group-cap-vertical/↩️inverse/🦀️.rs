//! ↩️ `change-pile-group-cap-vertical` inverse: restores the previous `cap_vertical`.

use crate::change_pile_group_cap_vertical::ChangePileGroupCapVertical;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileGroupCapVertical, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_group_cap_vertical::ChangePileGroupCapVertical> {
    Ok(match base.pile_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileGroupCapVertical { id: payload.id.clone(), new_cap_vertical: row.cap_vertical })],
        None => Vec::new(),
    })
}
