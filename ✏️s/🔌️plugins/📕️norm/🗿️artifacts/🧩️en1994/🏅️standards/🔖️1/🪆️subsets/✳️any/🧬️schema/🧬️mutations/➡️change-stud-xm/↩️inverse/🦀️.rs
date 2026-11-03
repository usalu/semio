//! ↩️ Inverse for `change-stud-xm`.
use crate::change_stud_x_m::ChangeStudXM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeStudXM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_stud_x_m::ChangeStudXM> {
    let Some(index) = base.studs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(stud) = base.studs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeStudXM { id: payload.id.clone(), new_x_m: stud.x_m })])
}
