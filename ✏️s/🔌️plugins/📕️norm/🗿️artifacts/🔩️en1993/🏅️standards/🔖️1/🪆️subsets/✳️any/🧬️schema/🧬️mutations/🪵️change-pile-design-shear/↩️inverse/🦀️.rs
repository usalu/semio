//! ↩️ `change-pile-design-shear` inverse: restores the previous `design_shear`.

use crate::change_pile_design_shear::ChangePileDesignShear;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileDesignShear, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_design_shear::ChangePileDesignShear> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileDesignShear { id: payload.id.clone(), new_design_shear: row.design_shear })],
        None => Vec::new(),
    })
}
