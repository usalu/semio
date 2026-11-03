//! ↩️ `change-silo-boundary-factor-hoop` inverse: restores the previous `boundary_factor_hoop`.

use crate::change_silo_boundary_factor_hoop::ChangeSiloBoundaryFactorHoop;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloBoundaryFactorHoop, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_boundary_factor_hoop::ChangeSiloBoundaryFactorHoop> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloBoundaryFactorHoop { id: payload.id.clone(), new_boundary_factor_hoop: row.boundary_factor_hoop })],
        None => Vec::new(),
    })
}
