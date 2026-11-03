//! ↩️ Inverse for `change-beam-patch-bearing-length-m`.
use crate::change_beam_patch_bearing_length_m::ChangeBeamPatchBearingLengthM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamPatchBearingLengthM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_patch_bearing_length_m::ChangeBeamPatchBearingLengthM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamPatchBearingLengthM { id: payload.id.clone(), new_patch_bearing_length_m: beam.patch_bearing_length_m })])
}
