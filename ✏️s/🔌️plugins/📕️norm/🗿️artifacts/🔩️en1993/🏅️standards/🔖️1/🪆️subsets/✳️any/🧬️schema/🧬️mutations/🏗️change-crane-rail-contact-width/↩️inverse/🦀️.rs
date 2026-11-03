//! ↩️ `change-crane-rail-contact-width` inverse: restores the previous `rail_contact_width`.

use crate::change_crane_rail_contact_width::ChangeCraneRailContactWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneRailContactWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_rail_contact_width::ChangeCraneRailContactWidth> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneRailContactWidth { id: payload.id.clone(), new_rail_contact_width: row.rail_contact_width })],
        None => Vec::new(),
    })
}
