//! ↩️ Inverse for `change-retaining-wall-permeability-ms`.
use crate::change_retaining_wall_permeability_m_s::ChangeRetainingWallPermeabilityMS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeRetainingWallPermeabilityMS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_retaining_wall_permeability_m_s::ChangeRetainingWallPermeabilityMS> {
    Ok(match base.retaining_walls.get(payload.index) {
        Some(row) => vec![Op::from(ChangeRetainingWallPermeabilityMS { index: payload.index, new_permeability_m_s: row.permeability_m_s })],
        None => Vec::new(),
    })
}
