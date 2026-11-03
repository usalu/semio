//! ↩️ `change-crane-rail-inertia` inverse: restores the previous `rail_inertia`.

use crate::change_crane_rail_inertia::ChangeCraneRailInertia;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneRailInertia, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_rail_inertia::ChangeCraneRailInertia> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneRailInertia { id: payload.id.clone(), new_rail_inertia: row.rail_inertia })],
        None => Vec::new(),
    })
}
