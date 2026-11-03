//! ↩️ `change-tower-guy-area` inverse: restores the previous `guy_area`.

use crate::change_tower_guy_area::ChangeTowerGuyArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerGuyArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_guy_area::ChangeTowerGuyArea> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerGuyArea { id: payload.id.clone(), new_guy_area: row.guy_area })],
        None => Vec::new(),
    })
}
