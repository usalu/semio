//! ↩️ Inverse for `change-beam-ltb-length-m`.
use crate::change_beam_ltb_length_m::ChangeBeamLtbLengthM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamLtbLengthM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_ltb_length_m::ChangeBeamLtbLengthM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamLtbLengthM { id: payload.id.clone(), new_ltb_length_m: beam.ltb_length_m })])
}
