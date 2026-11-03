//! ↩️ `change-bridge-design-life` inverse: restores the previous `design_life`.

use crate::change_bridge_design_life::ChangeBridgeDesignLife;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDesignLife, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_design_life::ChangeBridgeDesignLife> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeDesignLife { id: payload.id.clone(), new_design_life: row.design_life })],
        None => Vec::new(),
    })
}
