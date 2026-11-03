use crate::change_uls_equation::ChangeUlsEquation;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(_payload: &ChangeUlsEquation, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_uls_equation::ChangeUlsEquation> {
    Ok((|| {
    vec![Op::from(ChangeUlsEquation { new_value: base.uls_equation })]

    })())
}
