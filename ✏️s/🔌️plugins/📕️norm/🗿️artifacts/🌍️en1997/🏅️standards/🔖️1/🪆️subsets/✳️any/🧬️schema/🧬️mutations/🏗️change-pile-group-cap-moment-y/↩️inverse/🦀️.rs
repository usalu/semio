//! ↩️ `change-pile-group-cap-moment-y` inverse: restores the previous `cap_moment_y`.

use crate::change_pile_group_cap_moment_y::ChangePileGroupCapMomentY;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileGroupCapMomentY, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_group_cap_moment_y::ChangePileGroupCapMomentY> {
    Ok(match base.pile_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileGroupCapMomentY { id: payload.id.clone(), new_cap_moment_y: row.cap_moment_y })],
        None => Vec::new(),
    })
}
