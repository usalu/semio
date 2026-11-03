//! ↩️ `change-pile-design-life` inverse: restores the previous `design_life`.

use crate::change_pile_design_life::ChangePileDesignLife;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileDesignLife, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_design_life::ChangePileDesignLife> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileDesignLife { id: payload.id.clone(), new_design_life: row.design_life })],
        None => Vec::new(),
    })
}
