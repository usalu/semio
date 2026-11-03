//! ↩️ `change-crane-rail-radius` inverse: restores the previous `rail_radius`.

use crate::change_crane_rail_radius::ChangeCraneRailRadius;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneRailRadius, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_rail_radius::ChangeCraneRailRadius> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneRailRadius { id: payload.id.clone(), new_rail_radius: row.rail_radius })],
        None => Vec::new(),
    })
}
