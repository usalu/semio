//! ↩️ `change-t-int-c` inverse.

use super::ChangeTIntC;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeTIntC, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din4108Mutation::ChangeTIntC(ChangeTIntC { new_t_int_c: base.t_int_c })]

    })())
}
