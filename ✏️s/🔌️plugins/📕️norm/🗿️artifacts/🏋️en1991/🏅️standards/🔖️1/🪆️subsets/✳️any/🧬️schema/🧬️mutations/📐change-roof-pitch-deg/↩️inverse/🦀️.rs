//! ↩️ `change-roof-pitch-deg` inverse: restores the previous `pitch_deg`.

use crate::change_roof_pitch_deg::ChangeRoofPitchDeg;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRoofPitchDeg, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_pitch_deg::ChangeRoofPitchDeg> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRoofPitchDeg, base: &En1991Snapshot) -> Option<ChangeRoofPitchDeg> {
    let r0 = base.roofs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRoofPitchDeg { id: payload.id.clone(), new_pitch_deg: r0.pitch_deg })
}
