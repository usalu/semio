//! ↩️ `change-pile-shear-area` inverse: restores the previous `shear_area`.

use crate::change_pile_shear_area::ChangePileShearArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileShearArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_shear_area::ChangePileShearArea> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileShearArea { id: payload.id.clone(), new_shear_area: row.shear_area })],
        None => Vec::new(),
    })
}
