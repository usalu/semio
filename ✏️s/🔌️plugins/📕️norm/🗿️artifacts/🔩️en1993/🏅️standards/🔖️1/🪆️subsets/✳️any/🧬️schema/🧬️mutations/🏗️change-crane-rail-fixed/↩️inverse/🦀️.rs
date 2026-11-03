//! ↩️ `change-crane-rail-fixed` inverse: restores the previous `rail_fixed`.

use crate::change_crane_rail_fixed::ChangeCraneRailFixed;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneRailFixed, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_rail_fixed::ChangeCraneRailFixed> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneRailFixed { id: payload.id.clone(), new_rail_fixed: row.rail_fixed })],
        None => Vec::new(),
    })
}
