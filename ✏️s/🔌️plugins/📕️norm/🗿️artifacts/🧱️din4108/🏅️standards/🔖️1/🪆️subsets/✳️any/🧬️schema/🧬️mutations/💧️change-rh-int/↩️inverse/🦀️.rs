//! ↩️ `change-rh-int` inverse.

use super::ChangeRhInt;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeRhInt, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din4108Mutation::ChangeRhInt(ChangeRhInt { new_rh_int: base.rh_int })]

    })())
}
