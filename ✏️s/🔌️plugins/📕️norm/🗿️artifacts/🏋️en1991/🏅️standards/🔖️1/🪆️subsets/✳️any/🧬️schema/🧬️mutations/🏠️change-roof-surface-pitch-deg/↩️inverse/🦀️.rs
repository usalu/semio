//! ↩️ `change-roof-surface-pitch-deg` inverse: restores the previous `pitch_deg`.

use crate::change_roof_surface_pitch_deg::ChangeRoofSurfacePitchDeg;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofSurfacePitchDeg, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_surface_pitch_deg::ChangeRoofSurfacePitchDeg> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofSurfacePitchDeg, base: &En1991Snapshot) -> Option<ChangeRoofSurfacePitchDeg> {
    let r0 = base.roof_surfaces.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofSurfacePitchDeg { id: payload.id.clone(), new_pitch_deg: r0.pitch_deg })
}
