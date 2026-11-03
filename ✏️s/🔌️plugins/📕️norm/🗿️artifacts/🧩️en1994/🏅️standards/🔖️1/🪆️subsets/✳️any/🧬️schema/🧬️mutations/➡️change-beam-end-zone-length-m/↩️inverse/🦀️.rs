//! ↩️ Inverse for `change-beam-end-zone-length-m`.
use crate::change_beam_end_zone_length_m::ChangeBeamEndZoneLengthM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamEndZoneLengthM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_end_zone_length_m::ChangeBeamEndZoneLengthM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamEndZoneLengthM { id: payload.id.clone(), new_end_zone_length_m: beam.end_zone_length_m })])
}
