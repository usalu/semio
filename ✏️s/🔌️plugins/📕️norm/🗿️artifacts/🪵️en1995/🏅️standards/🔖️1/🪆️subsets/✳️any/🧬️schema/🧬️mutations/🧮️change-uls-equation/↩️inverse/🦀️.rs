use crate::change_uls_equation::ChangeUlsEquation;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(_payload: &ChangeUlsEquation, base: &En1995Snapshot) -> Option<crate::change_uls_equation::ChangeUlsEquation> {
    Some(ChangeUlsEquation { new_value: base.uls_equation })
}
