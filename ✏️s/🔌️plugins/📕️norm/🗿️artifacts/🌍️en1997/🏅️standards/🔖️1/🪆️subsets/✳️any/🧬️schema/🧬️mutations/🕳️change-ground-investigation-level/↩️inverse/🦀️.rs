//! ↩️ `change-ground-investigation-level` inverse: restores the previous `ground_level`.

use crate::change_ground_investigation_level::ChangeGroundInvestigationLevel;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeGroundInvestigationLevel, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ground_investigation_level::ChangeGroundInvestigationLevel> {
    Ok(match base.investigations.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeGroundInvestigationLevel { id: payload.id.clone(), new_ground_level: row.ground_level })],
        None => Vec::new(),
    })
}
