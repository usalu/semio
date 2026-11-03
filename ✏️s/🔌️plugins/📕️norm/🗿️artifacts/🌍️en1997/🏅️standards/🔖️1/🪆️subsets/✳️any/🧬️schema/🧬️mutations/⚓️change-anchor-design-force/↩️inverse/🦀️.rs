//! ↩️ `change-anchor-design-force` inverse: restores the previous `design_force`.

use crate::change_anchor_design_force::ChangeAnchorDesignForce;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorDesignForce, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_design_force::ChangeAnchorDesignForce> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorDesignForce { id: payload.id.clone(), new_design_force: row.design_force })],
        None => Vec::new(),
    })
}
