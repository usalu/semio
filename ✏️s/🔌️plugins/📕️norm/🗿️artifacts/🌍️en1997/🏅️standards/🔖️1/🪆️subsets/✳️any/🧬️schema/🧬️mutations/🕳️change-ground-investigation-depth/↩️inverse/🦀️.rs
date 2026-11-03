//! ↩️ `change-ground-investigation-depth` inverse: restores the previous `depth`.

use crate::change_ground_investigation_depth::ChangeGroundInvestigationDepth;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeGroundInvestigationDepth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ground_investigation_depth::ChangeGroundInvestigationDepth> {
    Ok(match base.investigations.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeGroundInvestigationDepth { id: payload.id.clone(), new_depth: row.depth })],
        None => Vec::new(),
    })
}
