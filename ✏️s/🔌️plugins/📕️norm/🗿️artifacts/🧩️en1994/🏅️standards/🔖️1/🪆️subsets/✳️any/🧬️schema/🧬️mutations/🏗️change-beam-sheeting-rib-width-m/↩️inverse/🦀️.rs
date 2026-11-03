//! ↩️ Inverse for `change-beam-sheeting-rib-width-m`.
use crate::change_beam_sheeting_rib_width_m::ChangeBeamSheetingRibWidthM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSheetingRibWidthM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_sheeting_rib_width_m::ChangeBeamSheetingRibWidthM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSheetingRibWidthM { id: payload.id.clone(), new_rib_width_m: beam.sheeting.rib_width_m })])
}
