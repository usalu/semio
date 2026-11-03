//! ↩️ `change-weld-eccentricity` inverse: restores the previous `eccentricity`.

use crate::change_weld_eccentricity::ChangeWeldEccentricity;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldEccentricity, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_eccentricity::ChangeWeldEccentricity> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldEccentricity { id: payload.id.clone(), new_eccentricity: row.eccentricity })],
        None => Vec::new(),
    })
}
