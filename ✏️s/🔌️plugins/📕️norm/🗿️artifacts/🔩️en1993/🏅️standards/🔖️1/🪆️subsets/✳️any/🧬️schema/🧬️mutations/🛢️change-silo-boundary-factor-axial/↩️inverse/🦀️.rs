//! ↩️ `change-silo-boundary-factor-axial` inverse: restores the previous `boundary_factor_axial`.

use crate::change_silo_boundary_factor_axial::ChangeSiloBoundaryFactorAxial;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloBoundaryFactorAxial, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_boundary_factor_axial::ChangeSiloBoundaryFactorAxial> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloBoundaryFactorAxial { id: payload.id.clone(), new_boundary_factor_axial: row.boundary_factor_axial })],
        None => Vec::new(),
    })
}
