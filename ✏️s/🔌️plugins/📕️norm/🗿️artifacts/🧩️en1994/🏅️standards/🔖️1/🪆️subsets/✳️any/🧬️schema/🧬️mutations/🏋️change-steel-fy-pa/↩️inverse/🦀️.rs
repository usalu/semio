//! Inverse for `change-steel-fy-pa`.
use super::ChangeSteelFYPa;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(_payload: &ChangeSteelFYPa, base: &En1994Snapshot) -> Result<Vec<En1994Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1994Mutation::ChangeSteelFYPa(ChangeSteelFYPa { new_steel_f_y_pa: base.steel_f_y_pa })]

    })())
}
