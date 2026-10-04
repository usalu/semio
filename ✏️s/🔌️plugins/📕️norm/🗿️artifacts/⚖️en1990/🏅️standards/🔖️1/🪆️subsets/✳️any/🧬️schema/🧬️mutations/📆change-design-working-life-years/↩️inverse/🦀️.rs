//! ↩️ `change-design-working-life-years` inverse.

use super::ChangeDesignWorkingLifeYears;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeDesignWorkingLifeYears, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangeDesignWorkingLifeYears(ChangeDesignWorkingLifeYears { new_design_working_life_years: base.design_working_life_years })]

    })())
}
