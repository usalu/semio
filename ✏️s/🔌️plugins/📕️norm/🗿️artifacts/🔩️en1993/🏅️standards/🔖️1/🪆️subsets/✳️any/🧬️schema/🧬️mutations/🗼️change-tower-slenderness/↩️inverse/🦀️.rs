//! ↩️ `change-tower-slenderness` inverse: restores the previous `slenderness`.

use crate::change_tower_slenderness::ChangeTowerSlenderness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerSlenderness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_slenderness::ChangeTowerSlenderness> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerSlenderness { id: payload.id.clone(), new_slenderness: row.slenderness })],
        None => Vec::new(),
    })
}
