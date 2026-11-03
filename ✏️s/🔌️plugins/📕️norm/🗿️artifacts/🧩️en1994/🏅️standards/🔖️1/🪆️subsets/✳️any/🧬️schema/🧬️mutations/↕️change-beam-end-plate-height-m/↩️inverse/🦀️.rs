//! ↩️ Inverse for `change-beam-end-plate-height-m`.
use crate::change_beam_end_plate_height_m::ChangeBeamEndPlateHeightM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamEndPlateHeightM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_end_plate_height_m::ChangeBeamEndPlateHeightM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamEndPlateHeightM { id: payload.id.clone(), new_end_plate_height_m: beam.end_plate_height_m })])
}
