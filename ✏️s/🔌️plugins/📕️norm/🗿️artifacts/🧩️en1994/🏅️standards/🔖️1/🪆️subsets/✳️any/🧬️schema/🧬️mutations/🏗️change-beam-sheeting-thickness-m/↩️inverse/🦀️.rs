//! ↩️ Inverse for `change-beam-sheeting-thickness-m`.
use crate::change_beam_sheeting_thickness_m::ChangeBeamSheetingThicknessM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSheetingThicknessM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_sheeting_thickness_m::ChangeBeamSheetingThicknessM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSheetingThicknessM { id: payload.id.clone(), new_thickness_m: beam.sheeting.thickness_m })])
}
