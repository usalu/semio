//! ↩️ `change-ground-investigation-kind` inverse: restores the previous `kind`.

use crate::change_ground_investigation_kind::ChangeGroundInvestigationKind;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeGroundInvestigationKind, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ground_investigation_kind::ChangeGroundInvestigationKind> {
    Ok(match base.investigations.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeGroundInvestigationKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
