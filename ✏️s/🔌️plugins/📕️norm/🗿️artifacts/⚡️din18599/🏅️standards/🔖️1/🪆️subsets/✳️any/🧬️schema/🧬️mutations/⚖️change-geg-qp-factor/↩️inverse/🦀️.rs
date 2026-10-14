//! ↩️ `change-geg-qp-factor` inverse.

use crate::change_geg_qp_factor::ChangeGegQpFactor;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ChangeGegQpFactor, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_geg_qp_factor::ChangeGegQpFactor> {
    Ok((|| {
    vec![Op::from(ChangeGegQpFactor { new_geg_qp_factor: base.geg_qp_factor })]

    })())
}
