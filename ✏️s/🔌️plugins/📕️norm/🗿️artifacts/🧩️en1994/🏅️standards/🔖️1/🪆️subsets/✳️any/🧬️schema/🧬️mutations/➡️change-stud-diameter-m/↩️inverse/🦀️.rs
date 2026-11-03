//! ↩️ Inverse for `change-stud-diameter-m`.
use crate::change_stud_diameter_m::ChangeStudDiameterM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeStudDiameterM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_stud_diameter_m::ChangeStudDiameterM> {
    let Some(index) = base.studs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(stud) = base.studs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeStudDiameterM { id: payload.id.clone(), new_diameter_m: stud.diameter_m })])
}
