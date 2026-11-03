//! ↩️ `change-foundation-soil-phi-deg` inverse: restores the previous `soil_friction_angle_deg`.

use crate::change_foundation_soil_phi_deg::ChangeFoundationSoilPhiDeg;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationSoilPhiDeg, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_soil_phi_deg::ChangeFoundationSoilPhiDeg> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationSoilPhiDeg, base: &En1998Snapshot) -> Option<ChangeFoundationSoilPhiDeg> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationSoilPhiDeg { id: payload.id.clone(), new_soil_friction_angle_deg: r0.soil_friction_angle_deg })
}
