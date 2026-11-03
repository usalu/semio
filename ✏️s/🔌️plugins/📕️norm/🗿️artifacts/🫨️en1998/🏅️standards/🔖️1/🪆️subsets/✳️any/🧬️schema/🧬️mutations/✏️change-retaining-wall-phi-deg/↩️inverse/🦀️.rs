//! ↩️ `change-retaining-wall-phi-deg` inverse: restores the previous `phi_deg`.

use crate::change_retaining_wall_phi_deg::ChangeRetainingWallPhiDeg;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeRetainingWallPhiDeg, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_retaining_wall_phi_deg::ChangeRetainingWallPhiDeg> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRetainingWallPhiDeg, base: &En1998Snapshot) -> Option<ChangeRetainingWallPhiDeg> {
    let r0 = base.retaining_walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRetainingWallPhiDeg { id: payload.id.clone(), new_phi_deg: r0.phi_deg })
}
