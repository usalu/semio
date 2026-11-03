//! ↩️ `change-ground-investigation-position-x` inverse: restores the previous `position_x`.

use crate::change_ground_investigation_position_x::ChangeGroundInvestigationPositionX;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeGroundInvestigationPositionX, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ground_investigation_position_x::ChangeGroundInvestigationPositionX> {
    Ok(match base.investigations.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeGroundInvestigationPositionX { id: payload.id.clone(), new_position_x: row.position_x })],
        None => Vec::new(),
    })
}
