//! ↩️ `change-anchor-inclination-deg` inverse: restores the previous `inclination_deg`.

use crate::change_anchor_inclination_deg::ChangeAnchorInclinationDeg;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorInclinationDeg, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_inclination_deg::ChangeAnchorInclinationDeg> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorInclinationDeg { id: payload.id.clone(), new_inclination_deg: row.inclination_deg })],
        None => Vec::new(),
    })
}
