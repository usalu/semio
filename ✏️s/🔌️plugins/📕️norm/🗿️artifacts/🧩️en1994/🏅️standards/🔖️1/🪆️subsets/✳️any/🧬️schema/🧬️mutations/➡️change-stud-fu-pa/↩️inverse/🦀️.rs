//! ↩️ Inverse for `change-stud-fu-pa`.
use crate::change_stud_f_u_pa::ChangeStudFUPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeStudFUPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_stud_f_u_pa::ChangeStudFUPa> {
    let Some(index) = base.studs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(stud) = base.studs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeStudFUPa { id: payload.id.clone(), new_f_u_pa: stud.f_u_pa })])
}
