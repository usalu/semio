//! ↩️ `change-pile-lateral-head-load` inverse: restores the previous `lateral_head_load`.

use crate::change_pile_lateral_head_load::ChangePileLateralHeadLoad;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileLateralHeadLoad, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_lateral_head_load::ChangePileLateralHeadLoad> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileLateralHeadLoad { id: payload.id.clone(), new_lateral_head_load: row.lateral_head_load })],
        None => Vec::new(),
    })
}
