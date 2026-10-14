//! ↩️ `change-theta-rm` inverse.
use crate::change_theta_rm::ChangeThetaRm;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(_payload: &ChangeThetaRm, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_theta_rm::ChangeThetaRm> {
    Ok((|| {
    vec![Op::from(ChangeThetaRm { new_theta_rm_c: base.theta_rm_c })]

    })())
}
