//! ↩️ Inverse for `change-stud-height-m`.
use crate::change_stud_height_m::ChangeStudHeightM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeStudHeightM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_stud_height_m::ChangeStudHeightM> {
    let Some(index) = base.studs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(stud) = base.studs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeStudHeightM { id: payload.id.clone(), new_height_m: stud.height_m })])
}
