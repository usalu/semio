//! ↩️ `change-geg-qp-factor` inverse.

use crate::mutations::change_geg_qp_factor::ChangeGegQpFactor;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeGegQpFactor, base: &Din18599Snapshot) -> Result<Vec<Din18599Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din18599Mutation::ChangeGegQpFactor(ChangeGegQpFactor { new_geg_qp_factor: base.geg_qp_factor })]

    })())
}
