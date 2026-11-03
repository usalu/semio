//! ↩️ `change-pile-shaft-perimeter` inverse: restores the previous `shaft_perimeter`.

use crate::change_pile_shaft_perimeter::ChangePileShaftPerimeter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileShaftPerimeter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_shaft_perimeter::ChangePileShaftPerimeter> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileShaftPerimeter { id: payload.id.clone(), new_shaft_perimeter: row.shaft_perimeter })],
        None => Vec::new(),
    })
}
