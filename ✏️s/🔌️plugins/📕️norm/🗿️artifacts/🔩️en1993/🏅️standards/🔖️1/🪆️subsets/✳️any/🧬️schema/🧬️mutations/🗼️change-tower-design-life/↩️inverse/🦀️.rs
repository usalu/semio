//! ↩️ `change-tower-design-life` inverse: restores the previous `design_life`.

use crate::change_tower_design_life::ChangeTowerDesignLife;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerDesignLife, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_design_life::ChangeTowerDesignLife> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerDesignLife { id: payload.id.clone(), new_design_life: row.design_life })],
        None => Vec::new(),
    })
}
