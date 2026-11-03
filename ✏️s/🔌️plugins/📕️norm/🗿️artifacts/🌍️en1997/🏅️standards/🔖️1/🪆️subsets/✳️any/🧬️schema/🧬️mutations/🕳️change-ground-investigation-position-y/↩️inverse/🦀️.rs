//! ↩️ `change-ground-investigation-position-y` inverse: restores the previous `position_y`.

use crate::change_ground_investigation_position_y::ChangeGroundInvestigationPositionY;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeGroundInvestigationPositionY, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ground_investigation_position_y::ChangeGroundInvestigationPositionY> {
    Ok(match base.investigations.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeGroundInvestigationPositionY { id: payload.id.clone(), new_position_y: row.position_y })],
        None => Vec::new(),
    })
}
