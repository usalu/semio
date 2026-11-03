//! ↩️ `change-pile-group-base-cohesion` inverse: restores the previous `base_cohesion`.

use crate::change_pile_group_base_cohesion::ChangePileGroupBaseCohesion;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileGroupBaseCohesion, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_group_base_cohesion::ChangePileGroupBaseCohesion> {
    Ok(match base.pile_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileGroupBaseCohesion { id: payload.id.clone(), new_base_cohesion: row.base_cohesion })],
        None => Vec::new(),
    })
}
