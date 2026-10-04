//! ↩️ `change-cellar-ventilation` inverse.
use super::ChangeCellarVentilation;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(_payload: &ChangeCellarVentilation, base: &Din16798Snapshot) -> Result<Vec<Din16798Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din16798Mutation::ChangeCellarVentilation(ChangeCellarVentilation { new_cellar_ventilation_m3_h: base.cellar_ventilation_m3_h })]

    })())
}
